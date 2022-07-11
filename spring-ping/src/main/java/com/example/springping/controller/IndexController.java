package com.example.springping.controller;

import org.springframework.web.bind.annotation.GetMapping;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RestController;


@RestController
public class IndexController {
    @PostMapping( "/")
    public String index(@RequestBody(required = false) String content) {
        return content;
    }
    @GetMapping( "/")
    public String index() {
        return "Hello secure!";
    }
}
