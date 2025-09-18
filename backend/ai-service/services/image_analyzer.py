"""
Image Analysis Service using OpenAI GPT-4V
"""

import os
import base64
import logging
from typing import Dict, Any, Optional
from io import BytesIO
from PIL import Image
import requests
from openai import AsyncOpenAI

logger = logging.getLogger(__name__)

class ImageAnalyzer:
    """Image analysis service using OpenAI GPT-4V"""
    
    def __init__(self):
        """Initialize the image analyzer"""
        self.client = AsyncOpenAI(
            api_key=os.getenv('OPENAI_API_KEY')
        )
        self.model = os.getenv('OPENAI_MODEL', 'gpt-4-vision-preview')
        
        if not os.getenv('OPENAI_API_KEY'):
            raise ValueError("OPENAI_API_KEY environment variable is required")
    
    async def analyze_image(
        self, 
        image_url: str, 
        analysis_type: str = "image_description",
        options: Optional[Dict[str, Any]] = None
    ) -> Dict[str, Any]:
        """
        Analyze image from URL
        
        Args:
            image_url: URL to the image
            analysis_type: Type of analysis to perform
            options: Additional analysis options
            
        Returns:
            Analysis results dictionary
        """
        try:
            # Download image
            response = requests.get(image_url, timeout=30)
            response.raise_for_status()
            
            return await self.analyze_image_content(
                image_content=response.content,
                analysis_type=analysis_type,
                options=options
            )
            
        except Exception as e:
            logger.error(f"Failed to analyze image from URL {image_url}: {e}")
            raise
    
    async def analyze_image_content(
        self,
        image_content: bytes,
        analysis_type: str = "image_description",
        options: Optional[Dict[str, Any]] = None
    ) -> Dict[str, Any]:
        """
        Analyze image content
        
        Args:
            image_content: Raw image bytes
            analysis_type: Type of analysis to perform
            options: Additional analysis options
            
        Returns:
            Analysis results dictionary
        """
        try:
            # Convert image to base64
            base64_image = base64.b64encode(image_content).decode('utf-8')
            
            # Get analysis prompt based on type
            prompt = self._get_analysis_prompt(analysis_type, options)
            
            # Call OpenAI API
            response = await self.client.chat.completions.create(
                model=self.model,
                messages=[
                    {
                        "role": "user",
                        "content": [
                            {
                                "type": "text",
                                "text": prompt
                            },
                            {
                                "type": "image_url",
                                "image_url": {
                                    "url": f"data:image/jpeg;base64,{base64_image}",
                                    "detail": options.get("detail_level", "auto") if options else "auto"
                                }
                            }
                        ]
                    }
                ],
                max_tokens=1000,
                temperature=0.1
            )
            
            # Extract and parse response
            content = response.choices[0].message.content
            
            # Try to extract structured data if possible
            result = self._parse_analysis_result(content, analysis_type)
            
            return {
                "analysis_type": analysis_type,
                "content": content,
                "structured_data": result,
                "model_used": self.model,
                "tokens_used": response.usage.total_tokens if response.usage else 0
            }
            
        except Exception as e:
            logger.error(f"Failed to analyze image content: {e}")
            raise
    
    def _get_analysis_prompt(self, analysis_type: str, options: Optional[Dict[str, Any]] = None) -> str:
        """Get analysis prompt based on type"""
        options = options or {}
        
        prompts = {
            "image_description": """
                Please provide a detailed description of this image. Include:
                - Main subjects and objects
                - Setting and environment
                - Colors and composition
                - Any text visible in the image
                - Overall mood or atmosphere
                
                Format your response as a clear, descriptive paragraph.
            """,
            
            "object_detection": """
                Identify and list all objects visible in this image. For each object, provide:
                - Object name
                - Approximate location (e.g., "top left", "center", "bottom right")
                - Size relative to the image (small, medium, large)
                - Any notable characteristics
                
                Format as a structured list.
            """,
            
            "text_extraction": """
                Extract all text visible in this image. Include:
                - The exact text content
                - Location of the text in the image
                - Font style or appearance if notable
                - Language if not English
                
                If no text is visible, respond with "No text detected".
            """,
            
            "scene_analysis": """
                Analyze the scene in this image. Provide:
                - Type of location/setting
                - Time of day (if determinable)
                - Weather conditions (if visible)
                - Activity or event taking place
                - Number of people (if any)
                - Overall context and purpose
                
                Be specific and detailed in your analysis.
            """,
            
            "color_analysis": """
                Analyze the colors in this image. Provide:
                - Dominant colors
                - Color scheme (warm, cool, monochromatic, etc.)
                - Color distribution
                - Any significant color contrasts
                - Overall color mood or feeling
                
                Use specific color names when possible.
            """,
            
            "emotion_analysis": """
                If there are people in this image, analyze:
                - Facial expressions and emotions
                - Body language
                - Overall mood of the scene
                - Emotional context
                
                If no people are visible, analyze the emotional tone conveyed by the image itself.
            """
        }
        
        base_prompt = prompts.get(analysis_type, prompts["image_description"])
        
        # Add any specific options to the prompt
        if options.get("include_confidence"):
            base_prompt += "\n\nPlease include confidence levels for your observations."
        
        if options.get("focus_area"):
            base_prompt += f"\n\nPay special attention to: {options['focus_area']}"
        
        return base_prompt.strip()
    
    def _parse_analysis_result(self, content: str, analysis_type: str) -> Dict[str, Any]:
        """Parse analysis result into structured data"""
        result = {}
        
        try:
            if analysis_type == "object_detection":
                # Try to extract objects from the response
                lines = content.split('\n')
                objects = []
                for line in lines:
                    line = line.strip()
                    if line and not line.startswith('#') and ':' in line:
                        parts = line.split(':', 1)
                        if len(parts) == 2:
                            objects.append({
                                "name": parts[0].strip(),
                                "description": parts[1].strip()
                            })
                result["objects"] = objects
            
            elif analysis_type == "text_extraction":
                # Try to identify extracted text
                if "no text" in content.lower():
                    result["text_found"] = False
                    result["extracted_text"] = []
                else:
                    result["text_found"] = True
                    # Simple extraction - could be improved with regex
                    result["extracted_text"] = [content]
            
            elif analysis_type == "color_analysis":
                # Try to extract color information
                colors = []
                color_words = ["red", "blue", "green", "yellow", "orange", "purple", "pink", 
                              "brown", "black", "white", "gray", "grey"]
                for color in color_words:
                    if color in content.lower():
                        colors.append(color)
                result["colors_mentioned"] = colors
            
            # Always include the full content
            result["full_analysis"] = content
            
        except Exception as e:
            logger.warning(f"Failed to parse structured data: {e}")
            result["full_analysis"] = content
        
        return result