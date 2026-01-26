import json
import os
from typing import List, Dict, Any

class DatapackageGenerator:
    SCHEMA_URL = "https://manyfold.app/profiles/0.0/datapackage.json"

    def __init__(self, name: str, title: str):
        self.data = {
            "$schema": self.SCHEMA_URL,
            "name": name,
            "title": title,
            "caption": "",
            "description": "",
            "keywords": [],
            "resources": [],
            "contributors": [],
            "links": []
        }

    def add_resource(self, name: str, path: str, mediatype: str):
        # Sanitize path: Ensure no Windows backslashes
        clean_path = path.replace("\\", "/")
        
        self.data["resources"].append({
            "name": name,
            "path": clean_path,
            "mediatype": mediatype,
            "up": "+z",
            "presupported": False
        })
    
    def set_description(self, description: str):
        self.data["description"] = description
    
    def add_contributor(self, name: str, role: str = "creator"):
        self.data["contributors"].append({
            "title": name,
            "roles": [role]
        })

    def write(self, output_path: str):
        with open(output_path, 'w', encoding='utf-8') as f:
            json.dump(self.data, f, indent=2)
