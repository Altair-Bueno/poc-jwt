package com.example.springping.controller;

import org.springframework.web.bind.annotation.*;


@RestController
@CrossOrigin
public class IndexController {
    @GetMapping(value = "/",consumes = "application/json", produces = "application/json")
    public String index() {
        return "Hello from Spring ping!";
    }
    @PostMapping(value = "/",consumes = "application/json", produces = "application/json")
    public String index(@RequestBody String content) {
        return content;
    }
}
