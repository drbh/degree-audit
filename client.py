import requests
import json

headers = {
    'Content-Type': 'application/json',
}


working_request = {
    "map": [
        [
            {
                "original": "Mathematics (MA)",
                "card": [
                        [
                            {
                                "match_type": "Group",
                                "group": "MA"
                            }
                        ]
                ]
            }
        ]
    ],
    "student": {
        "name": "drbh",
        "majors": [
                "art",
                "coffee"
        ],
        "classes": [
            {
                "when": 0,
                "grade": 0,
                "class": {
                    "hours": 10,
                    "subject": "SCI",
                    "level": 100,
                    "group": [
                        "MA",
                        "C",
                        "LA"
                    ]
                }
            }
        ]
    }
}


data = json.dumps(working_request)

## Make requests to hosted lambda
response = requests.post(
    'https://rkwy8keva8.execute-api.us-east-1.amazonaws.com/rustTest', headers=headers, data=data)

## Make requests to server
# response = requests.post('http://localhost:9966/audit',
#                          headers=headers, data=data)

obj = response.json()
print(json.dumps(obj, indent=4))
