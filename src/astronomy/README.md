# Astronomy

This is a library for environmental procedural content generation and related tools related to astronomy.

The general idea is that I want to be able to create maps, rooms, weather phenomena, geological features, astronomical features, constellations, that sort of thing.  Natural and artificial locations.  Tides!  Moons!  Crap like that.

I don't know, I'm just making this up as I go.

Currently we generate habitable planets from the stellar neighborhood level and work our way down the scale.  We start by generating roughly twelve star systems:

<img width="895" alt="Screen Shot 2022-10-16 at 4 31 01 PM" src="https://user-images.githubusercontent.com/1318579/196056830-0aea9951-64ad-406c-9e1b-6e470cb39013.png">

Each star system may be either a distant binary or single, each of which may be a close binary or a solitary.  Thus we can have a solitary host star, binary host star, or a combination of solitary and close binary stars in our system.  Around each host star are zero or more planets, and around each of those are zero or more moons.
