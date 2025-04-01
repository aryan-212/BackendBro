[
  {
    "route": "/",
    "is_route_dynamic": "false",
    "method": "get",
    "request_body": "None",
    "response": "string"
  },
  {
    "route": "/forex",
    "is_route_dynamic": "false",
    "method": "post",
    "request_body": {
      "from_currency": "string",
      "to_currency": "string"
    },
    "response": {
      "from_currency": "string",
      "to_currency": "string",
      "exchange_rate": "number"
    }
  }
]