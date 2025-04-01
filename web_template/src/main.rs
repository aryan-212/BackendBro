[
  {
    "route": "/pokemon",
    "is_route_dynamic": "false",
    "method": "post",
    "request_body": {
      "id": "number",
      "name": "string",
      "types": "array",
      "image_url": "string"
    },
    "response": "None"
  },
  {
    "route": "/pokemon",
    "is_route_dynamic": "false",
    "method": "get",
    "request_body": "None",
    "response": {
      "id": "number",
      "name": "string",
      "types": "array",
      "image_url": "string"
    }
  },
  {
    "route": "/pokemon",
    "is_route_dynamic": "false",
    "method": "put",
    "request_body": {
      "id": "number",
      "name": "string",
      "types": "array",
      "image_url": "string"
    },
    "response": "None"
  },
  {
    "route": "/pokemon/{id}",
    "is_route_dynamic": "true",
    "method": "get",
    "request_body": "None",
    "response": {
      "id": "number",
      "name": "string",
      "types": "array",
      "image_url": "string"
    }
  },
  {
    "route": "/pokemon/{id}",
    "is_route_dynamic": "true",
    "method": "delete",
    "request_body": "None",
    "response": "None"
  }
]