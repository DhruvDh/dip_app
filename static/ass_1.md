

<template>
    <div class="markdown-body">
      <h1>Assignment 1</h1>
      <ul>
      <li><a href="#assignment-1">Assignment 1</a>
      <ul>
      <li><a href="#introduction">Introduction</a></li>
      <li><a href="#a-quick-review">A quick review</a></li>
      <li><a href="#submission-template-and-rubric">Submission template and Rubric</a></li>
      </ul>
      </li>
      </ul>
      <h2>Introduction</h2>
      <p>The goal of this assignment is for you to get an intuition for the kind of raw data is produced by a camera sensor before it is turned into an image. To do so, I’ll be handing to you the kind of analog signal a camera would produce, for you to play around with. The assignment is open-ended, and you are free to choose how you wish to go about solving it - I’ll mention 3 or 4 different ways at the end here.</p>
      <p>So in this assignment you will be given 3 text files, each with a list of numbers in them - which represent the voltage produced at each pixel location for a CMOS sensor while taking a particular picture. The <code>_easy.txt</code> and the <code>_medium.txt</code> files represent a 256 by 256 image, while the <code>_hard.txt</code> represents a 512 by 512 pixel image. So a list of (256 * 256) voltage values for the <code>_easy.txt</code> and <code>_medium.txt</code> files, and (512 * 512) values for the <code>_hard.txt</code> file.</p>
      <p>Each of those “pictures” contain one of three objects - a ball, brick, or cylinder, like so ––</p>
      <p><img src="https://unccv.github.io/digital_image_processing/img/ball_brick_cylinder.gif" alt="Ball, brick, cylinder" /></p>
      <p>You are tasked with using what you’ve learnt about how different intensities producing different sort of voltage values, to try and write a program that would visualise the “signal” in the file you are given enough to be able to guess at what the object in your “image” is.</p>
      <p>In the files section, you will find an <code>assignment_1_files.zip</code> which will have a bunch of text files. The one’s meant for you will start with your NinerNet username. If your username is <code>ddhamani</code>, look for <code>ddhamani_easy.txt</code>, <code>ddhamani_medium.txt</code>, and <code>dhamani_hard.txt</code></p>
      <p>A couple of notes ––</p>
      <ul>
      <li>To make things easier for you, the pictures are grayscale, meaning that each voltage value for each pixel co-relates with the intensity of gray color at that pixel.</li>
      <li>Each student is assigned a random image - please don’t think that your friends’ <code>_easy.txt</code> file has a brick in it so yours’ will too. I wrote a script to randomly assign each student a random file.</li>
      <li>The easy images only have the object somewhere within them, the medium and hard ones may or may not have additional background objects.</li>
      <li>Since I am not really giving you guys a whole lot of instruction on how to go about doing this - the idea is to try and get you to think for yourself - this assignment is only worth 30 points, which is about a third of what an assignment is usually worth.</li>
      <li>You will not be graded on whether or not you were able to figure out what was inside the <code>_hard.txt</code> file. However if you do manage to do so, you will get some extra credit. Detailed rubric will be included.</li>
      <li>If you feel lost, feel free to ask for a hint on Zulip and one of your classmates will help you out. If quite a lot of people feel lost, one of the instructors will step in with a hint.</li>
      </ul>
      <h2>A quick review</h2>
      <ul>
      <li>
      <p>We know from the first lecture that a camera sensor for the most part is just a grid of individual pixel sensors.</p>
      </li>
      <li>
      <p>Each “pixel sensor” is a tiny photodiode that turns any incident photons on the photodiode into electric potential.</p>
      </li>
      <li>
      <p>This electric potential is read out by circuitry as described in the previous lecture, and eventually handed over to the analog front end, and eventually gets turned into an image.</p>
      </li>
      <li>
      <p>The assignment given sort of butts into this process and gives you the analog signal produced, for you to play around with, and guess what object is in your “image”.</p>
      </li>
      <li>
      <p>To do this, we would require different voltages to “look” different in order for us to visually determine what was inside the image.</p>
      <ul>
      <li>In previous semester, some students wrote code to generate text art (ascii art) from these voltages and successfully identified the object within.</li>
      <li>Some used conditional formatting found in spreadsheet software (like Excel) had tools that helped assign a different color to different range of numbers, which also led to successful identification of the object within. (To do this, they first converted the list  to a CSV file, and then openned them in Excel)</li>
      </ul>
      </li>
      <li>
      <p>Another random way we can go about doing the same ––</p>
      <ul>
      <li>
      <p>Write code to read the list of values you were given in your <code>_easy.txt</code>, <code>_medium.txt</code>, and <code>__hard.txt</code> files.</p>
      </li>
      <li>
      <p>Try to assign color to 2-4 range of values. For example <code>0.2-0.8</code> is light-gray, <code>0.8</code> and above is black. As many ranges as you feel appropriate.</p>
      </li>
      <li>
      <p>Replace each voltage value in the list, with an string containing HTML span of the color for that interval, with a single character inside, for example - <code>0.246642</code> according to the above would be <code>&lt;span style=&quot;color:gray&quot;&gt;@&lt;/span&gt;</code>. Various gray colors as understood by HTML can be found <a href="https://www.w3schools.com/colors/colors_shades.asp">here</a>.</p>
      </li>
      <li>
      <p>You will now have a list of strings, like so ––</p>
      <pre lang="html"><code>  [
          &quot;&lt;span style=&quot;color:gray&quot;&gt;@&lt;/span&gt;&quot;,
          &quot;&lt;span style=&quot;color:black&quot;&gt;@&lt;/span&gt;&quot;,
          ...
        ]
      </code></pre>
      </li>
      <li>
      <p>Based on this list, write out an HTML document. Do know that you would have to add a <code>&lt;br&gt;</code> tag to insert a new line at the end of every “row” for the image - after every 256 spans for the easy and medium files, and 512 spans for the hard file. Like so ––</p>
      <pre lang="html"><code>  &lt;html&gt;
          &lt;head&gt;
            &lt;title&gt;ITCS 3134 Assignment 1&lt;/title&gt;
          &lt;/head&gt;
          &lt;body&gt;
            &lt;span style=&quot;color:gray&quot;&gt;@&lt;/span&gt;
            &lt;span style=&quot;color:black&quot;&gt;@&lt;/span&gt;
            ...
            [254 more spans]
            &lt;br&gt;
            &lt;span style=&quot;color:gray&quot;&gt;@&lt;/span&gt;
            [and so on..]
          &lt;/body&gt;
        &lt;/html&gt;
      </code></pre>
      </li>
      </ul>
      </li>
      <li>
      <p>Opening this HTML document in a webpage should let you identify what object is inside, after zooming out.</p>
      </li>
      <li>
      <p>I have not done this and tried it out myself, so maybe it won’t work out all that well. Worth a shot though.</p>
      </li>
      </ul>
      <h2>Submission template and Rubric</h2>
      <p>The submission for this assignment is a single text file - preferably a markdown document - answering the following questions ––</p>
      <ul>
      <li>
      <p>What object was inside your <code>_easy.txt</code> file? (ball, brick, or cylinder) Attach a screenshot of your visualization, and any code you wrote to do the same. [30 points]</p>
      </li>
      <li>
      <p>What object was inside your <code>_medium.txt</code> file? (ball, brick, or cylinder) Attach a screenshot of your visualization, and any code you wrote to do the same. [20 points]</p>
      </li>
      <li>
      <p>What object was inside your <code>_hard.txt</code> file? (ball, brick, or cylinder) Attach a screenshot of your visualization, and any code you wrote to do the same. It is unlikely any of the aforementioned methods would let you guess at what object was in there, to attempt this question you’d hopefully will need to do a little research. [10 extra credits]</p>
      </li>
      </ul>
      <p>Thank you, and do not hesitate to ask for help.</p>
      
    </div>
</template>
    

<style lang="scss">
    @import "@primer/css/markdown/index.scss";
</style>
    
    