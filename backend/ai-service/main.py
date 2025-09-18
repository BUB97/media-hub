#!/usr/bin/env python3
"""
AI Service for Media Hub
Provides AI analysis capabilities using LangChain and OpenAI GPT-4V
"""

import os
import logging
from contextlib import asynccontextmanager
from dotenv import load_dotenv

# Load environment variables
load_dotenv()

# Configure logging
logging.basicConfig(
    level=logging.INFO,
    format='%(asctime)s - %(name)s - %(levelname)s - %(message)s'
)
logger = logging.getLogger(__name__)

# Import services after environment setup
from fastapi import FastAPI, HTTPException, UploadFile, File
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
from typing import List, Optional
import json

# Global services
openai_client = None

@asynccontextmanager
async def lifespan(app: FastAPI):
    """Application lifespan manager"""
    global openai_client
    
    try:
        # Initialize OpenAI client
        import openai
        openai_client = openai.OpenAI(api_key=os.getenv("OPENAI_API_KEY"))
        logger.info("OpenAI client initialized successfully")
        
        yield
    except Exception as e:
        logger.error(f"Failed to initialize services: {e}")
        yield
    finally:
        logger.info("Shutting down services")

# Create FastAPI app
app = FastAPI(
    title="Media Hub AI Service",
    description="AI-powered media analysis and search service",
    version="1.0.0",
    lifespan=lifespan
)

# Add CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Pydantic models
class AnalysisRequest(BaseModel):
    analysis_id: str
    media_id: str
    media_url: str
    analysis_type: str
    options: Optional[dict] = {}

class AnalysisResponse(BaseModel):
    analysis_id: str
    media_id: str
    result: dict
    success: bool

class SimilaritySearchRequest(BaseModel):
    query: str
    limit: int = 10
    threshold: float = 0.7

class SimilaritySearchResponse(BaseModel):
    query: str
    results: List[dict]
    total: int
    success: bool
    error: Optional[str] = None
    timestamp: str

@app.get("/health")
async def health_check():
    """Health check endpoint"""
    return {"status": "healthy", "service": "ai-service"}

@app.post("/analyze/image", response_model=AnalysisResponse)
async def analyze_image(request: AnalysisRequest):
    """Analyze image content using GPT-4V"""
    try:
        if not openai_client:
            raise HTTPException(status_code=503, detail="OpenAI client not initialized")
        
        # Simple image analysis using OpenAI
        response = openai_client.chat.completions.create(
            model="gpt-4o-mini",
            messages=[
                {
                    "role": "user",
                    "content": [
                        {"type": "text", "text": f"Analyze this image for {request.analysis_type}"},
                        {"type": "image_url", "image_url": {"url": request.media_url}}
                    ]
                }
            ],
            max_tokens=300
        )
        
        result = {
            "description": response.choices[0].message.content,
            "analysis_type": request.analysis_type
        }
        
        return AnalysisResponse(
            analysis_id=request.analysis_id,
            media_id=request.media_id,
            result=result,
            success=True
        )
    
    except Exception as e:
        logger.error(f"Image analysis failed: {e}")
        return AnalysisResponse(
            analysis_id=request.analysis_id,
            media_id=request.media_id,
            result={"error": str(e)},
            success=False
        )

@app.post("/search/similarity", response_model=SimilaritySearchResponse)
async def similarity_search(request: SimilaritySearchRequest):
    """Perform similarity search"""
    try:
        # Simple mock response for now
        results = [
            {
                "media_id": "test_1",
                "similarity_score": 0.95,
                "content": "Test content 1",
                "metadata": {
                    "type": "image",
                    "tags": ["test", "sample"],
                    "created_at": "2025-09-17T14:00:00Z"
                }
            },
            {
                "media_id": "test_2", 
                "similarity_score": 0.85,
                "content": "Test content 2",
                "metadata": {
                    "type": "document",
                    "tags": ["test", "example"],
                    "created_at": "2025-09-17T13:30:00Z"
                }
            }
        ]
        
        return SimilaritySearchResponse(
            query=request.query,
            results=results,
            total=len(results),
            success=True,
            error=None,
            timestamp="2025-09-17T14:22:00Z"
        )
    
    except Exception as e:
        logger.error(f"Similarity search failed: {e}")
        raise HTTPException(status_code=500, detail=str(e))

if __name__ == "__main__":
    import uvicorn
    host = os.getenv('AI_SERVICE_HOST', '127.0.0.1')
    port = int(os.getenv('AI_SERVICE_PORT', 8001))
    
    uvicorn.run(
        "main:app",
        host=host,
        port=port,
        reload=True,
        log_level="info"
    )