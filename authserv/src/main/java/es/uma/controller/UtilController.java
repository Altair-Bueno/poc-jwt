package es.uma.controller;

import org.springframework.security.core.Authentication;
import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/util")
public class UtilController {
    @GetMapping("/auth")
    @PostMapping("/auth")
    public void secured(Authentication authentication) {
        System.out.println(authentication);
    }

    @GetMapping("/health")
    @PostMapping("/health")
    public void healthCheck() {}
}
