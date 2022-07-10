package es.uma.controller;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/util")
public class UtilController {
    @GetMapping("/auth")
    @PostMapping("/auth")
    public void secured() {}

    @GetMapping("/health")
    @PostMapping("/health")
    public void healthCheck() {}
}
