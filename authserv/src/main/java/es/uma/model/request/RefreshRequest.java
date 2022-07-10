package es.uma.model.request;

import lombok.Value;

import java.util.UUID;

@Value
public class RefreshRequest {
    UUID refreshToken;
}
