import json
import time
import subprocess
import argparse


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

        if entry is None:
            continue

        entries.append(entry)

    return entries


def execute(entry):
    subprocess.run('ydotool type "cd ~/animals\\n"', shell=True)
    time.sleep(1)

    subprocess.run('ydotool type "clear\\n"', shell=True)
    time.sleep(1)

    name = entry["name"]
    name_command = f'ydotool type "echo \\"{name}\\"\\n"'
    subprocess.run(name_command, shell=True)

    for cmd in entry["commands"]:
        full_command = f'ydotool type "{cmd}\\n"'

        subprocess.run(full_command, shell=True)
        time.sleep(2)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "-i",
        "--index",
        type=int,
        help="Execute only the command at this index",
    )
    args = parser.parse_args()

    entries = load_commands()

    if args.index is not None:
        if args.index < 0 or args.index >= len(entries):
            print(
                f"Invalid index {args.index}. "
                f"Valid range: 0-{len(entries)-1}"
            )
            return

        entries = [entries[args.index]]

    print("Focus the terminal in 5 seconds")
    time.sleep(5)

    subprocess.run('ydotool type "cd\\n"', shell=True)

    for entry in entries:
        print(f"\n=== {entry['name']} ===")
        execute(entry)
        time.sleep(5)


if __name__ == "__main__":
    main()
