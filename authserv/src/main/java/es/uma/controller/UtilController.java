package es.uma.controller;

import lombok.extern.java.Log;
import org.springframework.security.core.Authentication;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

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
