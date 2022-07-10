package es.uma.model.request;

import lombok.Builder;
import lombok.Value;

@Value
@Builder
public class LoginRequest {
    String username;
    String password;
}
