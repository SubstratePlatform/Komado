# Journal

## 2026-11-24

So heyo world, this is Komado

This may end up becoming a journal of sorts, but I don't know yet. I just wanted to write something down, and this seemed like a good place to do it. Basically Komado is currently my idea for a PNG decoder that uses less than 10 KB of RAM. It's very possible, just hard... and slow.

This is probably going to eventually be more than just PNG, and become a sort of Symphonia for images but embedded friendly and efficient from the get go, but for now it's just PNG.

I think supporting metadata too is a good idea, so that's another crate.

But back to PNG

I hear you screaming at the screen, "WHY AND HOW IS THIS POSSIBLE?!?!?!?!?!?!?" Well well well, I'm insane, and I don't take no for an answer. I also have read the spec, and they've literally pointed out how to do it efficiently, but no one has done it yet. So I will.

### Challenge #1: DEFLATE's 32 KBs

So. The first challenge is that DEFLATE, the compression algorithm used in PNG, has a 32 KB sliding window. This means that to decompress a PNG file, you need to keep track of the last 32 KB of uncompressed data. This is a problem for us because we want to use less than 10 KB of RAM... I thought that was obvious by now. Well how do we lie to both the image and algorithm? Well well well that's where my "Library of Babel" comes in. Basically, since the bytes are filtered pixels, since we already drew the previous pixels, we can rebuild them, and store offsets.

If that made no sense, here's a simpler explanation:

PNG goes in, pixels come out.
The pixels that come out are modified, and we can rebuild the original pixels from them.
We can then store the offsets of the original pixels, and use those offsets to rebuild the original pixels **when we need them**.
This way, we don't need to store the original pixels in memory, and we can just store the offsets.

## Challenge Dos: remembering the filters

So to rebuild an old byte, we need to know which of the 5 filters was used to make the row (yes it varies per row, and i understand why they did it but I hate having to make it myself.)

That's a 3-bit note per row. Using some napkin math the worst case I got is a 1 pixel wide grey image. Each row would be 2 stream bytes (1 filter, 1 pixel). So, a 32 KB window would be 16384 rows. Multiply that by 3 you get 49152 bits, or 6144 bytes. Plus ~1 KB for
Huffman state. Still under 10 KB!!!!

So if someone has a skinny image and it's 1 pixel wide and that... tall, firstly i have some questions for your life choices and sanity, and secondly, I can still handle it. So that's peam!

### Challenge Trois: Adam7

I'm not sure why there are 7 of them, but Adam7 (the 7th, /silly) is a pretty smart way to interlace an image. It allows you to display a low quality version of the image WHILE it's still downloading, and then improves the quality PROGRESSIVELY as more stuff comes in.

Really smart actually, wow, but, yet again, my life suffers cause I have do it myself...

---

Tangent, but people really should develop on older hardware or FOR older hardware/weaker hardware (or embedded systems lol), because it makes you think, hey... how can I optimize this such that:

- it uses less RAM
- it doesn't unnecessarily use more resources than it needs to
- it can run on weaker hardware
- optionally it can run on embedded devices
- I'm squeezing every last bit of performance out of this, and I can do it better than the next guy WHILE making it maintainable and not a pain to work with?

Okay some of those are a bit of a thing, but the point is, if you use hardware that's weaker than your target consumer and people, you'll be forced to make it work good on WEAKER hardware, and that will make it work smoothly and just better on the target, win-win!

Rant over

---

Okay so now, back to the Seventh Adam.

Because Adam7 draws in passes, the row "above" could be like in the middle of the image, or it could be at the beginning, or at the end, and so on. We don't know. So we gotta map from stream offset to a pass, row, and canvas spot. This isn't too bad, but just painful math.
