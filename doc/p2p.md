.

A running Armada instance (I will refer to it as a "node") by design only serves RPC traffic according to JSON-RPC API. Standalone node is a sink for data: it pulls the necessary data from the Feeder Gateway (blocks, state updates and classes) and then stores it (and indexes it as well) in a way that allows efficient lookup accorting to the JSON-RPC API spec. Armada itself does not produce any new data, so if no data is received from the gateway (I will refer to it as "source"), then the node will not update it's state, as pretty much all the API is effectively read-only.

So for now, there are two concepts: "node" and "source", that together bring a smallest and simplest possible peer-to-peer (there is only one peer) setup (diagram below). It is important to distinguish this state as further improvements are going to be build on top of it.

```
   ___________                  ___________    
 /             \              /             \  
|               |            |               | 
|    SOURCE     |  ------->  |     NODE      | 
|               |            |               | 
 \ ___________ /              \ ___________ /  

```
