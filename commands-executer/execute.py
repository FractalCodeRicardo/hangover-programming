import json
import time
import subprocess


def get_command(data):
    name = data.get("name")
    commands = data.get("commands", [])

    return {
        "name": name,
        "commands": commands,
    }


def load_commands():
    entries = []

    with open("commands.json", "r", encoding="utf-8") as f:
        json_file = json.load(f)

    for item in json_file:
        entry = get_command(item)

        if (entry is None):
            continue

        entries.append(entry)

    return entries


def execute(entry):
    for cmd in entry["commands"]:
        full_command = "ydotool type --key-delay 100 \"" + cmd + "\""
        subprocess.run(full_command, shell=True)
        time.sleep(0.5)
        subprocess.run("ydotool key 28:1 28:0", shell=True)


def main():
    entries = load_commands()

    for entry in entries:
        print(f"\n=== {entry['name']} ===")
        execute(entry)


if __name__ == "__main__":
    main()
