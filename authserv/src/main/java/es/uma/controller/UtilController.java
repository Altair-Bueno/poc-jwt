package es.uma.controller;

import lombok.extern.java.Log;
import org.springframework.security.core.Authentication;
import org.springframework.web.bind.annotation.*;

// Enable CORS request
@CrossOrigin
@RestController
@RequestMapping("/util")
@Log
public class UtilController {
    @GetMapping("/auth")
    @PostMapping("/auth")
    public void secured(Authentication authentication) {
        log.info("Received ping message with authentication " + authentication);
    }

    @GetMapping("/health")
    @PostMapping("/health")
    public void healthCheck() {}
}
