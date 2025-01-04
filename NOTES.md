- We have to specify "Clone" all over the place when dealing with Events. Can we simplify this somehow? Make it more expicit?
- I'm currently using a single enum for all events across all aggregates in a domain. This lets us have a single event publishing
  instance, and it probably means we can stick with a single events table in any database we use. This is sticking very closely to
  the Python implementation. In the future we should look at ways to do this differently...should we have separate tables for each
  aggregate? I'm not sure.