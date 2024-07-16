# Descension

Taken from your home as a young child. Stolen from sight and mind. Brought up to become a great force of power, you see not what beings pull strings from the shadows. Find those who lurk in the shadows and forge a new life from the one that was given to you.

Descend, and Arise once more.

## Compatibility

> [!WARNING]
> Due to the rapid development seen by some of the utilized libraries, this table is maintained to track when major version updates occur and ensure interoperabiliity between libraries (as some may not respect `SemVer`).

> [!CAUTION]
> As I am new to the concept and the documentation appears to be oriented towards libraries, the guidelines for `SemVer` may be broken unintentionally at times. In case an ammendment is not made, or made on time, please always reference this table to ensure no errors occur as a direct result.

| descension | rust | bevy | bevy_ecs_ldtk | LDtk | bevy_mod_aseprite | aseprite |
| --- | --- | --- | --- | --- | --- | --- |
| dev (0.0.x series) | 1.79.0 | 0.13 | main | 1.5.3 | 0.7 | 1.3.7 |
| 0.0.0-dev | 1.79.0 | 0.13 | - | - | - | - |

## About & History

Descension is a pet project of two high-school students. We both took a computer science course which sought to introduce students to the world of programming using Python + Pygame.


I, [@Cutieguwu](https://github.com/Cutieguwu), had a fairly substantial amount of prior experience with Python. Originally I had given up because I got sick of using turtle draw (turtle draw sucks and I hate making colourful spirals with a passion). A year or two before the course, I got back into the realm of basic high-level programming in Python.

Unlike my classmates, my struggle came from being so used to the changes introduced in Python 3.6 (f-strings, `pip` integration by default) and 3.10 (match case pattern matching \<3), and being dependant on them. The course meanwhile used Python 3.3.1... but I got an exemption so long as I maintained backwards compatability (I was failing to build a python version from over a decade ago on my Linux machines). I ended up failing that due to stupidity (trying to except the routine to use `if` instead of `match` if required, but the compiler rightfully got confused by `match` regardless), but I could still identify and explain the needed changes.

My summative game's code was miles ahead of my classmates due to my prior knowledge and oriented itself to being a backend framework. Where I wrote a few thousand lines building a super flexible framework and semi-documented API standards with as much reuseability and efficiency as I could possibly give it, my classmates made much more visually appealing games that were actually fun to play, rather than being a bunch of functionality tests and demos.

I am envious of that. I am no fun.

I am the single worst person to code beside. Your code sucks; do better. Where my code sucks is just temporary ;). /hj /nm

Where I sought efficiency and standardization (maybe my yet unconfirmed OCD), my friends sought fun. And very much to their credit, they took and built upon the teacher and I's pointers and gave their games their all. Given the few months they had to learn a text-based language with just a few hours of Scratch under their belts, they progressed a very long way.


[@JellieJayde](https://github.com/JellieJayde), one of my friends on the course, ended up on the other end of our class' spectrum of backend to frontend oriented code. Where I was on the total backend of the scale (pun intended), JellieJayde was the queen of the frontend. Her game was agreed upon to be the most enjoyable and visually appealling of the summative games written.

Almost all the graphics of the game were custom, rather than just copied images from the internet. Only a couple were modified from the net, for the memes.


What started as a joke that my backend skills would mesh brilliantly with JellieJayde's frontend development, became a reality, and this project was started.

But, with us being ambitious fools, we wanted to push ourselves. I, worried about Python + Pygame being too inefficient for us to achieve our goals, whatever they may become, began to learn Rust + Bevy + ECS paradigm. As our idea grew, so did our idea of what we wanted the frontend to look like.

Now with a growing idea of the frontend, we began to look for solutions. While looking around, we found an API to use LDtk. As for animations, we found bevy_mod_aseprite. Willing to spend some money, we got ourselves Aseprite. This is ideal as LDtk also has direct support for Aseprite. So, now we also needed to learn Aseprite + LDtk + bevy_ecs_ldtk + bevy_mod_aseprite.

And here we are.