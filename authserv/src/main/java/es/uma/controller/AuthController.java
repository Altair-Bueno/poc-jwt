package es.uma.controller;

import es.uma.model.LoginRequest;
import es.uma.model.RefreshRequest;
import es.uma.model.RegisterRequest;
import es.uma.model.Session;
import es.uma.service.AuthService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

@RestController
@RequestMapping("/auth")
public class AuthController {
    @Autowired
    AuthService authService;

    @PostMapping("/login")
    public Session login(@RequestBody LoginRequest loginRequest) {
        return authService.login(loginRequest);
    }
    @PostMapping("/register")
    public void register(@RequestBody RegisterRequest registerRequest) {
        authService.register(registerRequest);
    }

    @PostMapping("/refresh")
    public String refresh(@RequestBody RefreshRequest refreshRequest) {
        // TODO
        return "Foo";
    }
}
