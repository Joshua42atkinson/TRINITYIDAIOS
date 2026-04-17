import os

temp_file = "/home/joshua/Workflow/desktop_trinity/trinity-genesis/docs/grep_synthesis_temp.md"
main_file = "/home/joshua/Workflow/desktop_trinity/trinity-genesis/docs/EVERYTHING_WE_KNOW_ABOUT_LONGCAT_ON_AMD.md"

with open(temp_file, 'r', encoding='utf-8') as f:
    data = f.read()

with open(main_file, 'a', encoding='utf-8') as f:
    f.write('\n\n## 6. RAW EMPIRICAL DATA REPOSITORY\n\n')
    f.write('*The following is mechanically extracted directly from all workspace architecture files, filtering exclusively for LongCat and vLLM facts to establish pure engineering alignment.*\n\n')
    f.write(data)

os.remove(temp_file)
