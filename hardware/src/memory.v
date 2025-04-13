module memory (
    input             memRq,
    input             readNotWrite,
    input      [15:0] addr,
    input      [15:0] dataIn,
    output reg [15:0] dataOut,
    output reg [ 5:0] led
);
    reg [15:0] memory[31:0];
    wire [4:0] mem_addr = addr[4:0];

    always @(*) begin
        if (memRq) begin
            if (readNotWrite) begin
                dataOut = memory[mem_addr];
            end else begin
                memory[mem_addr] = dataIn;
            end
        end else begin
            dataOut = 8'hFF;
        end
        led = ~dataIn[5:0];
    end
endmodule
