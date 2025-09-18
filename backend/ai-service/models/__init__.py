"""
Models package for AI service
"""

from .analysis_models import (
    AnalysisRequest,
    AnalysisResponse,
    SimilaritySearchRequest,
    SimilaritySearchResult,
    SimilaritySearchResponse,
    ImageAnalysisOptions,
    EmbeddingRequest,
    EmbeddingResponse
)

__all__ = [
    "AnalysisRequest",
    "AnalysisResponse", 
    "SimilaritySearchRequest",
    "SimilaritySearchResult",
    "SimilaritySearchResponse",
    "ImageAnalysisOptions",
    "EmbeddingRequest",
    "EmbeddingResponse"
]