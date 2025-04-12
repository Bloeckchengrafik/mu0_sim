`default_nettype none


/**
 * @brief The comm module is used to control the uart communication and
 * internal execution state machine.
 *
 * The protocol is defined as follows:
 * First, transmit a command:
 * - p: Ping, just sends back a pong (P)
 * - r: Read memory, sends back the full memory values
 * - w: Write memory, writes the value specified in the next 64 bytes to the ram
 * - x: Execute code, start at 0
 * - s: Get the status of the execution state machine (+/-)
 */
module comm (
    input clk,
    input [7:0] dataIn,
    input [511:0] memory,
    input byteSending,
    output reg byteReady,
    output reg byteReadyOut,
    output reg [7:0] dataOut
);
    localparam COMM_STATE_IDLE = 0;
    localparam COMM_STATE_RXMEM = 2;
    localparam COMM_STATE_EXSTART = 3;
    localparam COMM_STATE_EXSTATE = 4;
    localparam COMM_STATE_TXMEM = 5;

    localparam MAX_MEMORY_COUNT = 63;

    initial begin
        byteReadyOut = 0;
        dataOut = 0;
    end

    reg [3:0] commState = COMM_STATE_IDLE;
    reg byteReceived = 0;
    reg [7:0] memCounter = 0;

    always @(posedge clk) begin
        if (byteReady && !byteSending && !byteReceived) begin
            byteReceived <= 1'b1;

            case (commState)
                COMM_STATE_IDLE: begin
                    case (dataIn)
                        "p": begin
                            byteReadyOut <= 1'b1;
                            dataOut <= "P";
                        end
                        "r": begin
                            commState <= COMM_STATE_TXMEM;
                            memCounter <= 0;
                            byteReadyOut <= 1'b1;
                            dataOut <= "R";
                        end
                        default: begin
                            byteReadyOut <= 1'b1;
                            dataOut <= "?";
                        end
                    endcase
                end

                COMM_STATE_TXMEM: begin
                    if (memCounter < MAX_MEMORY_COUNT) begin
                        byteReadyOut <= 1'b1;
                        dataOut <= {memory[memCounter*8+:8]};
                        memCounter <= memCounter + 1;
                    end else begin
                        commState <= COMM_STATE_IDLE;
                        byteReadyOut <= 1'b1;
                        dataOut <= "E";
                    end
                end
            endcase
        end

        if (byteSending) begin
            byteReadyOut <= 1'b0;
            byteReceived <= 1'b0;
        end
    end

endmodule
