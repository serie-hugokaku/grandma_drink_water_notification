# Grandma, Drink Water!! Notification
## Background
- My 78-year-old grandmother was hospitalized with heatstroke (but has since been discharged).
- I installed an air conditioner in her bedroom and used SwitchBot to manage the temperature in both the living room and bedroom.
    - My grandmother disliked the air conditioner.
## Challenges
- While I managed the temperature, I couldn't be sure she was staying hydrated.
    - She can't use a browser or smartphone apps, but she can use LINE.
## Solution
- I created a system that sends a LINE Notify message at regular intervals to encourage her to stay hydrated.
## Tech Stack
- Rust v1.79.0
- Cloud Run
- Cloud Build
- Cloud Scheduler

### Learnings
- I taught her how to use LINE's reaction feature, and she now reacts to the notifications.
    - This lets me know she's hydrated (although I already trust her).
- She said the notifications were too frequent every hour, so I changed it to every 2 hours, between 8am and 8pm.
- She seems to appreciate the notifications because it helps her keep track of time.