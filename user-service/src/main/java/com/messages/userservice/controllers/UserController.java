package com.messages.userservice.controllers;

import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

import com.messages.userservice.dto.AuthResponse;
import com.messages.userservice.dto.RegisterRequest;
import com.messages.userservice.services.UserService;

import lombok.RequiredArgsConstructor;

@RestController
@RequestMapping("/users")
@RequiredArgsConstructor
public class UserController {

    private final UserService userService;

    @PostMapping(value = "register_temporal")
    public ResponseEntity<AuthResponse> registerTemporalUser() {

        try {
            return ResponseEntity.ok(userService.registerTemporalUser());

        } catch (Exception e) {
            AuthResponse response = AuthResponse.builder()
                    .build();
            return ResponseEntity.badRequest().body(response);
        }
    }

}
