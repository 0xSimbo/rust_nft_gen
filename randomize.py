import os
import random
import json
import shutil
start_id = 0
end_id = 10
NAME_PREFIX = "#"

all_ids = [x for x in range(start_id,end_id+1)]
random.shuffle(all_ids)

# #We shuffled and now we reorganize
def randomize():
    for i in range(len(all_ids)):
        try:
            original_file_number = i
            new_file_number = all_ids[i]
            os.rename(f"build/images/{i}.png",f"build/images/{new_file_number}.pngb")
            os.rename(f"build/json/{i}.json",f"build/json/{new_file_number}.jsonr")
        except:
            print(f"error build/json/{i}.json")
            exit()
    for i in range(len(all_ids)):
            os.rename(f"build/images/{i}.pngb",f"build/images/{i}.png")
            os.rename(f"build/json/{i}.jsonr",f"build/json/{i}.json")
            file = open(f"build/json/{i}.json","r")
            json_obj = json.load(file)
            json_obj['image'] = f"ipfs://hash/{i}.png"
            json_obj['name'] = f"{NAME_PREFIX}{i}"
            file.close()
            file = open(f"build/json/{i}.json","w")
            json.dump(json_obj,file,indent=4)
            file.close()





def main():
    randomize()
    return

main()