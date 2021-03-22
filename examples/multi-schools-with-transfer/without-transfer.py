import requests
import json
from auditor import Auditor, transfer

student = {
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
        },        
        {
            "when": 0,
            "grade": 0,
            "class": {
                "hours": 10,
                "subject": "BOWL",
                "level": 300,
                "group": []
            }
        }
    ]
}

# load in both maps
mapa = json.load(open('school-a/ba-english.json', "r"))
mapb = json.load(open('school-b/ba-english.json', "r"))

a = Auditor()

# run audit on school a
resa = a._make_request(mapa, student)

# convert classes from school a credits to school b 
# mapa_to_mapb_conversion = {"SCI 100": "SCIEN 102"}
# student["classes"] = transfer(student, mapa_to_mapb_conversion)

# run audit on school b
resb = a._make_request(mapb, student)

# print results
# print(json.dumps(resa, indent=4))
# print(json.dumps(resb, indent=4))

# count how many requirements
n_reqs_for_a = len(resa)
n_reqs_for_b = len(resb)

# count how many requirements met
sum_passed = 0
for x in resa:
    if x.get("met_flag"):
        sum_passed += 1

sum_passed_b = 0
for x in resb:
    if x.get("met_flag"):
        sum_passed_b += 1

# print results
print(f"School A - English {sum_passed}/{n_reqs_for_a} ")
print(f"School B - English {sum_passed_b}/{n_reqs_for_b} ")