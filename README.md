# Backend Framework Benchmark

This is an experiment where I implemented the same backend logic using multiple web frameworks to benchmark their performance in an optimized container environment.

### Frameworks Used
- **Spring Boot** (Java)
- **Node.js** (JavaScript)
- **Rocket** (Rust)

---

## How to Use

1. **Clone the repository**:
   ```bash
   git clone <your-repo-url>  
   cd Backends  
   ```
2. **Run the benchmark script**:  
   ```bash
   ./runstat.sh
   ```
- This will start all backend containers using Docker Compose and launch live stats monitoring for each.

---

## Cleaning up  
- To stop all running containers and remove the associated Docker images, simply run:
   ```bash
   ./cleanall.sh
   ```  
## Note
- Make sure Docker and Docker Compose are installed on your system.  
- The benchmark uses docker stats to compare host resource usage across frameworks.
