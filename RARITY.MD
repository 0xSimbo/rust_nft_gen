 In the ```rank_nfts_by_rarity``` function, we calculate the rarity score for each NFT based on the attribute probabilities. The probability of an attribute appearing in an NFT is calculated by dividing the number of times that attribute appears in all NFTs by the total number of NFTs.

Here's a step-by-step explanation of the probability calculation:

First, we count the occurrences of each attribute across all NFTs and store them in the ```attribute_frequencies``` HashMap. The key for this HashMap is a string in the format ```trait_type:value```, and the value is the number of times this attribute occurs in the NFTs.

We store the attributes for each NFT in the ```nft_attributes``` HashMap. The key for this HashMap is the token ID, and the value is a vector of attributes in the format ```trait_type:value```.

We then iterate through the ```nft_attributes``` HashMap and calculate the rarity score for each NFT. The rarity score is the sum of the rarity scores for each attribute in the NFT.

To calculate the rarity score of an attribute, we first find its probability. The probability is calculated as:

```rust
let probability = *attribute_frequencies.get(attr).unwrap() as f64 / total_nfts;
``` 
Here, ```attribute_frequencies.get(attr).unwrap()``` gives us the frequency (number of occurrences) of the attribute, and total_nfts is the total number of NFTs. Dividing the frequency by the total number of NFTs gives us the probability of the attribute appearing in an NFT.

Once we have the probability, we calculate the rarity score for that attribute as the negative base-2 logarithm of the probability:

```rust
-probability.log2()
```

The base-2 logarithm (log2) is used because it measures the information content of an event in bits. The negative sign is used to make rarer attributes have higher scores. The rarer an attribute is, the lower its probability, and therefore, the higher its rarity score.

Finally, we sum up the rarity scores of all attributes in an NFT to get the total rarity score for that NFT.
By calculating the rarity score in this way, we ensure that NFTs with rarer attributes have higher rarity scores.