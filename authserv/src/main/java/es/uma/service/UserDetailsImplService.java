package es.uma.service;

import es.uma.entity.UserEntity;
import es.uma.model.User;
import es.uma.repository.UserRepository;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.security.core.userdetails.UserDetails;
import org.springframework.security.core.userdetails.UserDetailsService;
import org.springframework.security.core.userdetails.UsernameNotFoundException;
import org.springframework.stereotype.Service;

import java.util.ArrayList;
import java.util.Optional;

@Service
public class UserDetailsImplService implements UserDetailsService {
    @Autowired
    private UserRepository userRepository;

    @Override
    public UserDetails loadUserByUsername(String username) throws UsernameNotFoundException {
        Optional<UserEntity> optionalUser = userRepository.findUserEntityByUsername(username);
        if (optionalUser.isEmpty())
            throw new UsernameNotFoundException(username);

        var user = optionalUser.get();
        return new User(user.getId(), user.getUsername(), user.getPassword(), new ArrayList<>());
    }
}
