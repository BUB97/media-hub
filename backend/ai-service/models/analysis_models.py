"""
Data models for AI analysis service
"""

from typing import Dict, List, Optional, Any
from pydantic import BaseModel, Field
from datetime import datetime

class AnalysisRequest(BaseModel):
    """Request model for AI analysis"""
    analysis_id: str = Field(..., description="Unique analysis identifier")
    media_id: str = Field(..., description="Media file identifier")
    media_url: str = Field(..., description="URL to the media file")
    analysis_type: str = Field(..., description="Type of analysis to perform")
    options: Optional[Dict[str, Any]] = Field(default={}, description="Additional analysis options")
    user_id: Optional[str] = Field(None, description="User identifier")

class AnalysisResponse(BaseModel):
    """Response model for AI analysis"""
    analysis_id: str = Field(..., description="Analysis identifier")
    media_id: str = Field(..., description="Media file identifier")
    result: Dict[str, Any] = Field(..., description="Analysis results")
    success: bool = Field(..., description="Whether analysis was successful")
    error: Optional[str] = Field(None, description="Error message if failed")
    timestamp: datetime = Field(default_factory=datetime.utcnow, description="Analysis timestamp")

class SimilaritySearchRequest(BaseModel):
    """Request model for similarity search"""
    query: str = Field(..., description="Search query text")
    limit: int = Field(default=10, description="Maximum number of results")
    threshold: float = Field(default=0.7, description="Similarity threshold")
    user_id: Optional[str] = Field(None, description="User identifier for filtering")

class SimilaritySearchResult(BaseModel):
    """Individual similarity search result"""
    media_id: str = Field(..., description="Media file identifier")
    content: str = Field(..., description="Content that matched")
    similarity_score: float = Field(..., description="Similarity score")
    metadata: Dict[str, Any] = Field(default={}, description="Additional metadata")

class SimilaritySearchResponse(BaseModel):
    """Response model for similarity search"""
    query: str = Field(..., description="Original search query")
    results: List[SimilaritySearchResult] = Field(..., description="Search results")
    success: bool = Field(..., description="Whether search was successful")
    error: Optional[str] = Field(None, description="Error message if failed")
    timestamp: datetime = Field(default_factory=datetime.utcnow, description="Search timestamp")

class ImageAnalysisOptions(BaseModel):
    """Options for image analysis"""
    include_objects: bool = Field(default=True, description="Include object detection")
    include_text: bool = Field(default=True, description="Include text extraction")
    include_emotions: bool = Field(default=False, description="Include emotion analysis")
    include_colors: bool = Field(default=False, description="Include color analysis")
    detail_level: str = Field(default="medium", description="Analysis detail level: low, medium, high")

class EmbeddingRequest(BaseModel):
    """Request model for storing embeddings"""
    media_id: str = Field(..., description="Media file identifier")
    content: str = Field(..., description="Content to embed")
    metadata: Optional[Dict[str, Any]] = Field(default={}, description="Additional metadata")
    user_id: Optional[str] = Field(None, description="User identifier")

class EmbeddingResponse(BaseModel):
    """Response model for embedding operations"""
    media_id: str = Field(..., description="Media file identifier")
    success: bool = Field(..., description="Whether operation was successful")
    error: Optional[str] = Field(None, description="Error message if failed")
    timestamp: datetime = Field(default_factory=datetime.utcnow, description="Operation timestamp")