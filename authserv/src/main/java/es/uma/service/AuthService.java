package es.uma.service;

import es.uma.repository.UserRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.security.core.userdetails.User;
import org.springframework.security.core.userdetails.UserDetails;
import org.springframework.security.core.userdetails.UserDetailsService;
import org.springframework.security.core.userdetails.UsernameNotFoundException;
import org.springframework.stereotype.Service;

import java.util.ArrayList;

@Service
public class AuthService implements UserDetailsService {
    @Autowired
    private UserRepository userRepository;

    @Override
    public UserDetails loadUserByUsername(String username) throws UsernameNotFoundException {
        var optionalUser = userRepository.findUserEntityByUsername(username);

        if (optionalUser.isEmpty())
            throw new UsernameNotFoundException(username);

        var user = optionalUser.get();
        return new User(user.getUsername(), user.getPassword(), new ArrayList<>());
    }
}
