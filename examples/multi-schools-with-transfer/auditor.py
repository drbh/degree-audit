import requests
import json

class Auditor(object):
	"""docstring for Auditor"""
	def __init__(self):
		super(Auditor, self).__init__()

	def _make_request(self, mapx, student):
		headers = {
		    'Content-Type': 'application/json',
		}

		working_request = {
		    "map": mapx,
		    "student": student
		}

		data = json.dumps(working_request)

		# response = requests.post('https://rkwy8keva8.execute-api.us-east-1.amazonaws.com/rustTest', headers=headers, data=data)

		response = requests.post('http://localhost:9966/audit',
		                         headers=headers, data=data)

		return response.json()

def transfer(student, mapa_to_mapb_conversion):
    new_classes = []
    for educlass in student.get("classes"):
        # print(educlass)

        new_class = json.loads(json.dumps(educlass))
        sub = educlass.get("class").get("subject")
        lvl = educlass.get("class").get("level")

        for key in mapa_to_mapb_conversion.keys():
            if key == f"{sub} {lvl}":
                # update class for conversion
                new_sub, new_lvl = mapa_to_mapb_conversion[key].split(" ")
                new_class["class"]["subject"] = new_sub
                new_class["class"]["level"] = int(new_lvl)

        new_classes.append(new_class)
    return new_classes