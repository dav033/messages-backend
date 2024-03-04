package com.messages.userservice.services;

import org.springframework.http.HttpEntity;
import org.springframework.http.HttpHeaders;
import org.springframework.http.MediaType;
import org.springframework.http.ResponseEntity;
import org.springframework.stereotype.Service;
import org.springframework.web.client.RestTemplate;

import com.messages.userservice.dto.AuthResponse;
import com.messages.userservice.dto.RegisterRequest;
import com.messages.userservice.dto.TokenResponse;
import com.messages.userservice.models.User;
import com.messages.userservice.repositories.UserRepository;

import ch.qos.logback.core.subst.Token;
import lombok.RequiredArgsConstructor;

@Service
@RequiredArgsConstructor
public class UserService {
    private final UserRepository userRepository;
    private final RestTemplate rest;

    public AuthResponse registerTemporalUser() {
        try {
            HttpHeaders headers = new HttpHeaders();
            headers.setContentType(MediaType.APPLICATION_JSON);

            User temporalUser = new User();

            User user_response = userRepository.save(temporalUser);

            user_response.setName("user_" + user_response.getId());

            userRepository.save(user_response);

            AuthResponse authResponse = AuthResponse.builder()
                    .id(user_response.getId())
                    .name(user_response.getName())
                    .profile_image(user_response.getProfile_image())
                    .temporal(true)
                    .build();

            System.out.println("authResponse: " + authResponse);

            HttpEntity<AuthResponse> requestBody = new HttpEntity<>(authResponse, headers);

            ResponseEntity<TokenResponse> response = rest.postForEntity("http://localhost:3000/token/create",
                    requestBody,
                    TokenResponse.class);

            TokenResponse tokenResponse = response.getBody();

            authResponse.setToken(tokenResponse.getToken());

            return authResponse;

        } catch (Exception e) {
            throw new RuntimeException("Error registering user");
        }

    }
}
