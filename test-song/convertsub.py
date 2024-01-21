import json
from datetime import timedelta

def parse_time(num):
    nanoseconds = num * 5668.9342403628117913832199546485260770975056689342403628117
    seconds, nanoseconds = divmod(nanoseconds, 1e9)
    microseconds = nanoseconds // 1e3
    return timedelta(seconds=seconds, microseconds=microseconds)

def format_timedelta(duration):
    hours, remainder = divmod(duration.total_seconds(), 3600)
    minutes, seconds = divmod(remainder, 60)
    milliseconds = int((seconds % 1) * 1000)
    
    return f"{int(hours):02d}:{int(minutes):02d}:{int(seconds):02d},{milliseconds:03d}"


input_file = "input.json"
output_file = "mansao.srt"

with open(input_file, "r", encoding="utf-8") as json_file:
    data = json.load(json_file)



with open(output_file, "w", encoding="utf-8") as srt_file:
    verses = []
    previous = None
    for idx, entry in enumerate(data, start=1):
        start_time = parse_time(entry["TEMPO"])
        end_time = parse_time(entry["TEMPO"] + entry.get("TEMPO_PB", 1000))  # Use TEMPO_PB for duration
        if "LETRA" in entry:
            subtitle_text = entry["LETRA"].replace("\r\n", "\\N")  # Convert line breaks to '\N' for .srt format
        else:
            subtitle_text = ""

        if previous != None:
            previous["end"] = start_time
            verses.append(previous)

        previous = {
            "index": idx,
            "start": start_time,
            "text": subtitle_text,
        }

    if previous != None:
        previous["end"] = start_time
        verses.append(previous)

    srt_content = ""
    for verse in verses:
        srt_content += f"{verse['index']}\n"
        srt_content += f"{format_timedelta(verse['start'])} "
        srt_content += f"--> {format_timedelta(verse['end'])}\n"
        srt_content += f"{verse['text']}\n\n"
    srt_file.write(srt_content)

print(f"Conversion complete. Output saved to '{output_file}'.")