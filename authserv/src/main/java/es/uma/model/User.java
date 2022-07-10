package es.uma.model;


import lombok.Getter;
import org.springframework.security.core.GrantedAuthority;

import java.util.Collection;
import java.util.UUID;

public class User extends org.springframework.security.core.userdetails.User {
    @Getter
    UUID id;

    public User(UUID id, String username, String password, Collection<? extends GrantedAuthority> authorities) {
        super(username, password, authorities);
        this.id = id;
    }


}
