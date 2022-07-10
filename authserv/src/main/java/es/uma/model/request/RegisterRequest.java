package es.uma.model.request;

import lombok.Value;

@Value
public class RegisterRequest {
    String username;
    String password;
}
