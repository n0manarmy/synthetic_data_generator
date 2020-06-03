import json
import sys
import os

if(len(sys.argv) < 3):
        print("validate_json.py {file name} {pretty print true|false}")
        exit(-1)

count=0
file_name=sys.argv[1]
display_pretty=sys.argv[2]
with open(file_name) as f:
        for line in f:
                try:
                        count = count + 1
                        parsed = json.loads(line)
                        if display_pretty == "true":
                                print(json.dumps(parsed,indent=4, sort_keys=False))
                        if count % 1000 == 0:
                                print("Records processed: " + str(count))
                except ValueError:
                        print("Error in parsing value at line below:")
                        print(line)
                        #print(json.dumps(parsed, indent=4, sort_keys=False))
