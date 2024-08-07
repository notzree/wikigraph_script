# Wikigraph script
> Rust script to 100x compress wikipedia xml dump into a traversable link graph
## Installation:
After cloning the repo, install the wikipedia xml dumps from [here] (https://dumps.wikimedia.org/enwiki/), download the enwiki-version-you-want-pages-articles.xml.bz2. Unzip the file with:
```
bunzip2 enwiki-version-you-want-pages-articles.xml.bz2
```
Then, make sure it's in the raw_data directory (or you can customize the directory variable). Also make sure to populate a .env file with the databse credentials of your choosing. Then, you can startup the docker container with
```
docker compose run wikigraph
//run this inside the docker container
cargo run
```
You should start to see a progress bar and an ETA.

Converts Wikipedia's XML Database dumps into a graph stored in a binary format. Inspired by: Tristan Hume's [Wikicrush](https://github.com/trishume/wikicrush). This borrows the binary format that Tristan described in the Readme of Wikicrush, which is highly compact and compresses the almost 100GB Wikipedia XML dump into a ~ 1.27GB Binary link graph. During development, I used the smaller simple english wiki, which I could process in ~6-8 minutes on my local machine.
## File format:
The file format contains a File header, a page header, and the links. Each header is represented by 4 32-bit integers. The file header has 2 unused integers, 1 integer representing the version, and 1 integer representing the number of pages (also called node in my code). The page header contains 3 unused integers which are used for marking visited nodes in traversal, as well as the number of links that the page has. Each link is a single integer that contains the byteoffset of the page it is linking to. This lets you skip to the next page by incrementing (4 * num_links) bytes forward. This also lets you easily access the page that is linked by moving the reader to the byteoffset. 
## How it works:
The script runs in 2 sections. The first section, it uses [quick_xml](https://docs.rs/quick-xml/latest/quick_xml/) to read through the dump and tries to parse all of the valid links from each page. It will append this data into a text adjacency list, which is used later on to reconstruct the binary graph. It also computes the byteoffsets and lengths of each valid page and stores it in a postgres database. 

For example: Anarchism has a byteoffset of 16 and a length of 1536 (all in bytes). This can be interpreted as: Anarchism is 16 bytes from the start of the file and has 1536/4 = 384 links. After this pre-processing stage is finished, The graph will get constructed from the adjacency list by reading the database into memory (around 2.5gbs) and then replacing the strings with byteoffsets. 

The completed .bin file can be traversed by adapting any pathfinding algorithim to the file format. In the [wikigraph server](wikigraph_server) it uses a simple BFS to compute the shortest path. The algorithim is quite finnicky as the conversion between byteoffsets to integers can get confusing.

## Performance:
All runs were performed in a docker environment using 8gbs of ram and 6 M2 CPU cores. 

Speed results for pre-processing section:
- 12 hours using regex to extract links
- 5.24 hours using custom parsing function
  
Speed results for graph-building section:
- 42 minutes after caching (Estimated almost 190+ hours before!)


## Caveats:
I was not able to successfully parse all links and as such this graph is not 100% fully complete. I ran into an issue differentiating capitalized and lowercased pages. For example, the programming language [ALGOL](https://en.wikipedia.org/wiki/ALGOL) and the star [Algol](https://en.wikipedia.org/wiki/Algol) are differentiated by the casing. This works fine as long as links from other pages that references these pages obey the same capitalization convention, this wasn't the case. I kept running into casing issues resulting in duplicate key errors or not resulting in entries being found in the database. I do intend on polishing this in the future (maybe during summer when I have no school). 




