package es.uma.controller;

import es.uma.model.request.LoginRequest;
import es.uma.model.request.RefreshRequest;
import es.uma.model.request.RegisterRequest;
import es.uma.model.response.CheckResponse;
import es.uma.model.response.Session;
import es.uma.service.AuthService;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.security.core.Authentication;
import org.springframework.security.core.GrantedAuthority;
import org.springframework.web.bind.annotation.*;

import java.util.stream.Collectors;

// Enable CORS request
@CrossOrigin
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
    public Session refresh(@RequestBody RefreshRequest refreshRequest) {
        return authService.refresh(refreshRequest);
    }
    @GetMapping("/check")
    public CheckResponse get(Authentication authentication) {
        var authorities = authentication.getAuthorities()
                .stream()
                .map(GrantedAuthority::getAuthority)
                .collect(Collectors.toList());
        return CheckResponse.builder()
                .name(authentication.getName())
                .authorities(authorities)
                .build();
    }
}
