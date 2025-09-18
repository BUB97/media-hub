"""
Vector Store Service using ChromaDB for similarity search
"""

import os
import logging
from typing import List, Dict, Any, Optional
import chromadb
from chromadb.config import Settings
from sentence_transformers import SentenceTransformer
import numpy as np

from models.analysis_models import SimilaritySearchResult

logger = logging.getLogger(__name__)

class VectorStoreService:
    """Vector store service for similarity search using ChromaDB"""
    
    def __init__(self):
        """Initialize the vector store service"""
        self.client = None
        self.collection = None
        self.embedding_model = None
        self.persist_directory = os.getenv('CHROMA_PERSIST_DIRECTORY', './chroma_db')
        self.embedding_model_name = os.getenv('EMBEDDING_MODEL', 'all-MiniLM-L6-v2')
        
    async def initialize(self):
        """Initialize ChromaDB and embedding model"""
        try:
            # Initialize ChromaDB client
            self.client = chromadb.PersistentClient(
                path=self.persist_directory,
                settings=Settings(
                    anonymized_telemetry=False,
                    allow_reset=True
                )
            )
            
            # Get or create collection
            self.collection = self.client.get_or_create_collection(
                name="media_embeddings",
                metadata={"description": "Media content embeddings for similarity search"}
            )
            
            # Initialize embedding model
            self.embedding_model = SentenceTransformer(self.embedding_model_name)
            
            logger.info(f"Vector store initialized with {self.collection.count()} embeddings")
            
        except Exception as e:
            logger.error(f"Failed to initialize vector store: {e}")
            raise
    
    async def store_embedding(
        self, 
        media_id: str, 
        content: str, 
        metadata: Optional[Dict[str, Any]] = None
    ):
        """
        Store content embedding in the vector database
        
        Args:
            media_id: Unique identifier for the media
            content: Text content to embed
            metadata: Additional metadata to store
        """
        try:
            if not self.collection or not self.embedding_model:
                raise RuntimeError("Vector store not initialized")
            
            # Generate embedding
            embedding = self.embedding_model.encode(content).tolist()
            
            # Prepare metadata
            doc_metadata = {
                "media_id": media_id,
                "content_length": len(content),
                **(metadata or {})
            }
            
            # Store in ChromaDB
            self.collection.add(
                embeddings=[embedding],
                documents=[content],
                metadatas=[doc_metadata],
                ids=[media_id]
            )
            
            logger.info(f"Stored embedding for media_id: {media_id}")
            
        except Exception as e:
            logger.error(f"Failed to store embedding for {media_id}: {e}")
            raise
    
    async def similarity_search(
        self, 
        query: str, 
        limit: int = 10, 
        threshold: float = 0.7,
        user_id: Optional[str] = None
    ) -> List[SimilaritySearchResult]:
        """
        Perform similarity search
        
        Args:
            query: Search query text
            limit: Maximum number of results
            threshold: Minimum similarity threshold
            user_id: Optional user ID for filtering
            
        Returns:
            List of similarity search results
        """
        try:
            if not self.collection or not self.embedding_model:
                raise RuntimeError("Vector store not initialized")
            
            # Generate query embedding
            query_embedding = self.embedding_model.encode(query).tolist()
            
            # Prepare where clause for filtering
            where_clause = {}
            if user_id:
                where_clause["user_id"] = user_id
            
            # Perform search
            results = self.collection.query(
                query_embeddings=[query_embedding],
                n_results=limit,
                where=where_clause if where_clause else None,
                include=["documents", "metadatas", "distances"]
            )
            
            # Process results
            search_results = []
            if results['documents'] and results['documents'][0]:
                for i, (doc, metadata, distance) in enumerate(zip(
                    results['documents'][0],
                    results['metadatas'][0],
                    results['distances'][0]
                )):
                    # Convert distance to similarity score (ChromaDB uses cosine distance)
                    similarity_score = 1.0 - distance
                    
                    # Filter by threshold
                    if similarity_score >= threshold:
                        search_results.append(SimilaritySearchResult(
                            media_id=metadata.get('media_id', f'unknown_{i}'),
                            content=doc,
                            similarity_score=similarity_score,
                            metadata=metadata
                        ))
            
            logger.info(f"Similarity search returned {len(search_results)} results for query: {query[:50]}...")
            return search_results
            
        except Exception as e:
            logger.error(f"Similarity search failed: {e}")
            raise
    
    async def delete_embedding(self, media_id: str):
        """Delete embedding by media ID"""
        try:
            if not self.collection:
                raise RuntimeError("Vector store not initialized")
            
            self.collection.delete(ids=[media_id])
            logger.info(f"Deleted embedding for media_id: {media_id}")
            
        except Exception as e:
            logger.error(f"Failed to delete embedding for {media_id}: {e}")
            raise
    
    async def update_embedding(
        self, 
        media_id: str, 
        content: str, 
        metadata: Optional[Dict[str, Any]] = None
    ):
        """Update existing embedding"""
        try:
            # Delete existing embedding
            await self.delete_embedding(media_id)
            
            # Store new embedding
            await self.store_embedding(media_id, content, metadata)
            
        except Exception as e:
            logger.error(f"Failed to update embedding for {media_id}: {e}")
            raise
    
    async def get_collection_stats(self) -> Dict[str, Any]:
        """Get collection statistics"""
        try:
            if not self.collection:
                raise RuntimeError("Vector store not initialized")
            
            count = self.collection.count()
            
            return {
                "total_embeddings": count,
                "collection_name": self.collection.name,
                "embedding_model": self.embedding_model_name,
                "persist_directory": self.persist_directory
            }
            
        except Exception as e:
            logger.error(f"Failed to get collection stats: {e}")
            raise
    
    async def close(self):
        """Close the vector store connection"""
        try:
            # ChromaDB client doesn't need explicit closing
            # but we can clear references
            self.collection = None
            self.client = None
            self.embedding_model = None
            
            logger.info("Vector store connection closed")
            
        except Exception as e:
            logger.error(f"Error closing vector store: {e}")
    
    async def search_by_media_id(self, media_id: str) -> Optional[Dict[str, Any]]:
        """Get stored content by media ID"""
        try:
            if not self.collection:
                raise RuntimeError("Vector store not initialized")
            
            results = self.collection.get(
                ids=[media_id],
                include=["documents", "metadatas"]
            )
            
            if results['documents'] and results['documents'][0]:
                return {
                    "media_id": media_id,
                    "content": results['documents'][0],
                    "metadata": results['metadatas'][0]
                }
            
            return None
            
        except Exception as e:
            logger.error(f"Failed to get content for {media_id}: {e}")
            raise