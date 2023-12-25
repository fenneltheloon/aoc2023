# Combining two map vectors

- Start with one source, one dest vector
- For each Map in dest vector:
  - Get list of all Maps in source that overlap by checking source.dest, remove them from source map (sort and use binary search)
  - For each that overlaps: create union map and add to final vector
  - Create the remainder map (partial source map that does not overlap) and add it back to source map
  - **Important:** There cannot be two overlapping maps in a single layer.
- Once all maps in dest have been processed, copy any remaining maps from source into final and sort by source start. 
