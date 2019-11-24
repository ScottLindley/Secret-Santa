# Secret Santa 🎅

I'm a software engineer. So naturally, that means that when I go home for the holidays, I have to fix a printer. This year, I was excited when I was given a different challenge.

My family celebrates Christmas, and we wanted to cut down on the number of gifts any one person is responsible for by doing what's called a "secret Santa". In case you don't know, that simply means each person is assigned a single giftee and each person only has one gifter assigned to them. Each participant is only supposed to know who they are assigned, and nothing else -- this puts the "secret" in "secret Santa". Of course, because I'll be the one running the program, I'll see who has who, but I'll try not to peek.

It's not uncommon to pick names out of a hat to decide who has who. But since we're all a little spread out geographically, it's hard for us all to be in one room to do so. So my Mom suggested that I write a little program to pick names at random for us.

Easy, I thought, it'll take me ten minutes. As it turns out, this was a more intersting problem then I had first anticipated.

---

My first thought was to put all the names in one list and for each person, pick one at random.

```
[
  scott,
  maggie,
  jordan,
  emily,
  derrick,
  arianna,
  dennis,
  carol,
]
```

But obviously this has some issues -- what if someone were to pick themselves? Also, we need to keep track of who has already been selected as each person is only allowed to receive one gift. One other important constraint that my Mom placed on the problem is: **"you're not allowed to pick your spouse/significant other"**. This adds a new level of complexity to the problem.

We could keep a separate list of "invalid" picks for each iteration of a loop and if an invalid pick is selected, we simply pick another random number and try again.

On paper, this works. But it produces a non-deterministic run-time as there is no guarantee that the same invalid pick won't be selected, say, a million times in a row (in theory, there's a small chance the program would never complete).

One tweak we could make is to temporarily remove each invalid pick from the list should one be selected, and then return them back to the list before the next turn. This means the algorithm's run-time is no longer non-deterministic, but it still feels... wrong. Why roll the dice against a set that contains invalid options at all?

Another approach I had initially thought of was to construct the list of names as a list of pairs, grouping partners together. This way, all invalid options for each person (self and partner) are in one place.

```
[
  (scott, maggie),
  (jordan, emily),
  (derrick, arianna),
  (dennis, carol),
]
```

Then for each person, select a pair at random and then select at random a person within that pair. But not only does this method suffer from the same pitfalls as the above approach, it is also not truly random. Let's say we start with Scott. Scott randomly selects one couple out of the three other couples. The couples have the same chance of being picked (1/3). And each person within each couple has a 1/2 chance, giving each person an overall chance of 1/6 of being selected. Let's imagine he selects `(dennis, carol)`. Then within that pair, he selects `carol` at random and we mark her as unavailable for future picks.
Next, it's Maggie's turn to pick. But wait a second, since `dennis` is the only one left in his couple, he has a 100% chance of being selected should Maggie pick his couple. This skews the randomness of our algorithm as `dennis` (1/3) has a higher likelihood of being selected over `arianna` (1/6).

---

### Graph it

Eventually, it occurred to me that we could represent this problem as a directional graph.

Let's imagine each person is a node in our graph, and each node has an outgoing edge to all other nodes **except** for themselves, and their partner (if they have one).

Here's a visual model of what those edges would look like for a single node.

![link-example](https://user-images.githubusercontent.com/21317789/69483714-bfd9e480-0df8-11ea-81a9-bef7aeffdcfa.png)

Scott (me) is married to Maggie, so he is not allowed to end up with her as a giftee. As you can see, there is neither an edge connecting Scott to Scott nor Scott to Maggie. In other words, the outgoing edges from the Scott node represent all valid options.

When we link up all nodes using these same rules, we end up with a graph that looks like the following:

![graph](https://user-images.githubusercontent.com/21317789/69483749-3a0a6900-0df9-11ea-9301-40b4b64318f0.png)

🤯 woah, looks complex! How do we traverse such a dense graph to find our solution?

---

### Algorithm

The algorithm is quite simple. Let's break it down.

We'll start with Scott and ignore all other edges for now.

![Secret-Santa-1](https://user-images.githubusercontent.com/21317789/69483940-6fb05180-0dfb-11ea-9f9b-0168c1dfc1f5.png)

Since we're only presented with valid options, we can safely pick one edge at random. Imagine that Scott's random pick is Emily. We can now remove all other outgoing edges *from* Scott since his options have collapsed into a single selection.

![Secret-Santa-2](https://user-images.githubusercontent.com/21317789/69483952-953d5b00-0dfb-11ea-83a6-853ef267761d.png)

Now, we want to be sure that Emily can't be selected by anyone else -- she is now an "invalid" option for all others. We'll want to examine all *incoming* edges into the Emily node.

![Secret-Santa-3](https://user-images.githubusercontent.com/21317789/69484013-1e549200-0dfc-11ea-8cb0-f039fac7c273.png)

And because Scott has claimed her as his giftee, we'll want to remove all other edges.

![Secret-Santa-4](https://user-images.githubusercontent.com/21317789/69487592-b7020680-0e2a-11ea-9a35-62b66a89dca8.png)


Then we move on to the next person to make a selection.

---

### Full example

Starting at Scott and moving clockwise, here's our initial state:

![graph](https://user-images.githubusercontent.com/21317789/69483749-3a0a6900-0df9-11ea-9301-40b4b64318f0.png)

Now we just move through and do the same thing for each person:
- select an outgoing edge at random
- remove all other outgoing edges
- follow the edge to the selected node
- remove all other incoming edges to the selected node


**Scott selects Emily**

![Secret-Santa-5](https://user-images.githubusercontent.com/21317789/69484076-ea2da100-0dfc-11ea-8d54-d95ae4ee79cf.png)


**Maggie selects Derrick**

![Secret-Santa-6](https://user-images.githubusercontent.com/21317789/69484077-ea2da100-0dfc-11ea-8d73-9a6e081aa698.png)


**Dennis selects Scott**

![Secret-Santa-7](https://user-images.githubusercontent.com/21317789/69484078-eac63780-0dfc-11ea-8c1f-29300ecb9838.png)


**Carol selects Arianna**

![Secret-Santa-8](https://user-images.githubusercontent.com/21317789/69484079-eac63780-0dfc-11ea-9784-94f599dbd941.png)


**Emily selects Maggie**

![Secret-Santa-9](https://user-images.githubusercontent.com/21317789/69484080-eac63780-0dfc-11ea-9ec9-4ff40ea7a07f.png)


**Jordan selects Carol**

![Secret-Santa-10](https://user-images.githubusercontent.com/21317789/69484081-eac63780-0dfc-11ea-952f-5f9479a72f93.png)


**Arianna selects Jordan**

![Secret-Santa-11](https://user-images.githubusercontent.com/21317789/69484082-eac63780-0dfc-11ea-88b8-69fbb915612d.png)

Now, by default, **Derrick selects Dennis** because it is the only outgoing edge left for his node.

Here, we have our solution! 🎉
Each node has exactly one incoming edge and one outgoing edge (one present they are gifting and one they are receiving).

Here's a gif that steps through each selection

![animation](https://user-images.githubusercontent.com/21317789/69487633-82427f00-0e2b-11ea-81b0-9ddbb86e08dd.gif)

--- 
There is one quirky scenario where it's possible for the last person to have no outgoing edges left. In this case we can assign the person to themselves and do one of two things:
- if the last person does **not** have a partner, then select anyone that picked before them at random and swap with them.
- else if the last person **does** have a partner, then swap with whoever their partner was assigned. This is guaranteed to be a valid pick and since their partner chose it at random, it is "fair".

---

### Code

At the time this request came in from my Mom, I was reading the [rust book](https://doc.rust-lang.org/stable/book/title-page.html) so I decided it would be fun to have a go at writing this in rust.

The program first reads in a config JSON file that expects a list of names and a "partner map". In the example above the config might look something like this:

```json
{
  "names": [
    "Scott",
    "Maggie",
    "Jordan",
    "Emily",
    "Derrick",
    "Arianna",
    "Dennis",
    "Carol"
  ],
  "partnerMap": {
    "Scott": "Maggie",
    "Maggie": "Scott",
    "Jordan": "Emily",
    "Emily": "Jordan",
    "Derrick": "Arianna",
    "Arianna": "Derrick",
    "Dennis": "Carol",
    "Carol": "Dennis"
  }
}
```

The use of a config file allows me to easily reuse the script for my wife's side of the family 😏.

From the config we then build a graph. I chose to represent this graph as a matrix. This made it easy to traverse down a column and "unlink" all "linked" edges for a given selection.

Here's a visual representation of what the matrix would look like at the initial and final states of the program. The matrix is simply a two dimensional array where rows and columns represent outgoing and incoming edges, respectively. The value in each cell denotes whether or not the nodes are connected by an edge.

|  Initial | Scott | Maggie | Jordan | Emily | Derrick | Arianna | Dennis | Carol |
|:--------:|:-----:|:------:|:------:|:-----:|:-------:|:-------:|:------:|:-----:|
|   Scott  |   0   |    0   |    1   |   1   |    1    |    1    |    1   |   1   |
|  Maggie  |   0   |    0   |    1   |   1   |    1    |    1    |    1   |   1   |
|  Jordan  |   1   |    1   |    0   |   0   |    1    |    1    |    1   |   1   |
|   Emily  |   1   |    1   |    0   |   0   |    1    |    1    |    1   |   1   |
|  Derrick |   1   |    1   |    1   |   1   |    0    |    0    |    1   |   1   |
|  Arianna |   1   |    1   |    1   |   1   |    0    |    0    |    1   |   1   |
|  Dennis  |   1   |    1   |    1   |   1   |    1    |    1    |    0   |   0   |
|   Carol  |   1   |    1   |    1   |   1   |    1    |    1    |    0   |   0   |

---
|   Final  | Scott | Maggie | Jordan | Emily | Derrick | Arianna | Dennis | Carol |
|:--------:|:-----:|:------:|:------:|:-----:|:-------:|:-------:|:------:|:-----:|
|   Scott  |   0   |    0   |    0   |   1   |    0    |    0    |    0   |   0   |
|  Maggie  |   0   |    0   |    0   |   0   |    1    |    0    |    0   |   0   |
|  Jordan  |   0   |    0   |    0   |   0   |    0    |    0    |    0   |   1   |
|   Emily  |   0   |    1   |    0   |   0   |    0    |    0    |    0   |   0   |
|  Derrick |   0   |    0   |    0   |   0   |    0    |    0    |    1   |   0   |
|  Arianna |   0   |    0   |    1   |   0   |    0    |    0    |    0   |   0   |
|  Dennis  |   1   |    0   |    0   |   0   |    0    |    0    |    0   |   0   |
|   Carol  |   0   |    0   |    0   |   0   |    0    |    1    |    0   |   0   |

---
### Wrap up

Curious how many possible permutations there are? Well, considering a single selection, choosing k number of items from a set of n with no repetitions can be expressed as

![Ck(n)=(kn)=k!(n−k)!n!](https://user-images.githubusercontent.com/21317789/69500467-4c53d800-0ec9-11ea-88e8-01b9b6abd5d1.png)

Since we're only choosing one edge at each step we can simplify the above to

![n](https://user-images.githubusercontent.com/21317789/69500472-5f66a800-0ec9-11ea-970f-763fde3b1616.png)

Now, each time we take a turn, n is decremented, so the series becomes

![1*2*3*4*...n](https://user-images.githubusercontent.com/21317789/69500476-68f01000-0ec9-11ea-8fd0-bdc9fbcc5f7e.png)

Or put simply,

![n!](https://user-images.githubusercontent.com/21317789/69500477-73aaa500-0ec9-11ea-8c34-b7140d5f00fc.png)

What is n? If we assume all participants have a partner, then n is the number of names - 2.
So for the provided example of 8 participants, each with a partner, there are 720 different permutations.

![P=(8-2)!](https://user-images.githubusercontent.com/21317789/69500478-74dbd200-0ec9-11ea-872f-91db4a6aae91.png)

After working through the problem, I realized this ended up being a [matching problem](https://en.wikipedia.org/wiki/Matching_(graph_theory)). I'm certainly not the first to come across one of these. Jack Edmonds details an algorithm for such problems in his 1965 paper ["Paths, Trees, and Flowers"](http://www.cs.mun.ca/~kol/courses/6743-w15/papers/edmonds65-blossom.pdf).

Happy holidays!
