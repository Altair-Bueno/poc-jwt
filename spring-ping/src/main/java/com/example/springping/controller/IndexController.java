package com.example.springping.controller;

import org.springframework.web.bind.annotation.*;


@RestController
@CrossOrigin
public class IndexController {
    @GetMapping("/")
    public String index() {
        return "Hello from Spring ping!";
    }
    @PostMapping("/")
    public String index(@RequestBody String content) {
        return content;
    }
}
